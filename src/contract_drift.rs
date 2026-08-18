//! Opt-in runtime contract-drift telemetry (TEL-001 / SUI-2366).
//!
//! Applications can observe soft provider-contract surprises (unknown enum
//! values, rejected off-origin pagination links) without panicking the SDK
//! path. Callbacks are **best-effort**, **redacted**, and isolated with
//! `catch_unwind` so a misbehaving observer never aborts request handling.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, RwLock};

/// Hard ceiling for free-form detail strings on signals (memory / log bound).
pub const CONTRACT_DRIFT_DETAIL_MAX_LEN: usize = 256;

/// Classification of a soft contract-drift observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ContractDriftKind {
    /// Provider sent an enum string the SDK does not map to a known variant.
    UnknownEnumValue,
    /// A HAL `next` href was ignored because its origin is not allowlisted.
    OffOriginPaginationLink,
}

/// One redacted drift observation suitable for metrics / logs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractDriftSignal {
    /// Drift category.
    pub kind: ContractDriftKind,
    /// Active operation id when known (request-scoped).
    pub operation: Option<&'static str>,
    /// Logical field or type path (never a secret).
    pub field_path: Option<&'static str>,
    /// Truncated, redacted detail (e.g. unknown enum raw or host).
    pub detail_redacted: String,
}

impl ContractDriftSignal {
    /// Builds a signal with detail truncated to [`CONTRACT_DRIFT_DETAIL_MAX_LEN`].
    pub fn new(
        kind: ContractDriftKind,
        operation: Option<&'static str>,
        field_path: Option<&'static str>,
        detail: impl AsRef<str>,
    ) -> Self {
        Self {
            kind,
            operation,
            field_path,
            detail_redacted: redact_detail(detail.as_ref()),
        }
    }
}

/// Observer for [`ContractDriftSignal`] emissions.
pub trait ContractDriftObserver: Send + Sync {
    /// Called when the SDK observes soft contract drift.
    ///
    /// Must not panic (panics are caught) and must not perform blocking I/O
    /// that would stall request completion for long.
    fn on_drift(&self, signal: &ContractDriftSignal);
}

/// Shared observer handle stored on clients.
pub type SharedContractDriftObserver = Arc<dyn ContractDriftObserver>;

/// No-op observer used in tests and as a default stand-in.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopContractDriftObserver;

impl ContractDriftObserver for NoopContractDriftObserver {
    fn on_drift(&self, _signal: &ContractDriftSignal) {}
}

std::thread_local! {
    static REQUEST_SCOPE: std::cell::RefCell<RequestDriftScope> =
        const { std::cell::RefCell::new(RequestDriftScope::empty()) };
}

#[derive(Clone, Default)]
struct RequestDriftScope {
    operation: Option<&'static str>,
    observer: Option<SharedContractDriftObserver>,
}

impl RequestDriftScope {
    const fn empty() -> Self {
        Self {
            operation: None,
            observer: None,
        }
    }
}

/// RAII guard that installs request-scoped drift context for the current thread.
pub struct ContractDriftScopeGuard {
    previous: RequestDriftScope,
}

impl ContractDriftScopeGuard {
    /// Installs operation + optional client observer for the duration of a request.
    pub fn enter(
        operation: &'static str,
        observer: Option<SharedContractDriftObserver>,
    ) -> Self {
        let previous = REQUEST_SCOPE.with(|cell| {
            let mut scope = cell.borrow_mut();
            let prev = scope.clone();
            *scope = RequestDriftScope {
                operation: Some(operation),
                observer,
            };
            prev
        });
        Self { previous }
    }
}

impl Drop for ContractDriftScopeGuard {
    fn drop(&mut self) {
        let previous = std::mem::take(&mut self.previous);
        REQUEST_SCOPE.with(|cell| {
            *cell.borrow_mut() = previous;
        });
    }
}

static GLOBAL_OBSERVER: RwLock<Option<SharedContractDriftObserver>> = RwLock::new(None);

/// Sets a process-wide fallback observer (overridden by client-scoped observers).
///
/// Prefer attaching an observer on [`crate::MollieClientBuilder`] so multi-tenant
/// processes do not share one sink. Pass `None` to clear.
pub fn set_global_contract_drift_observer(observer: Option<SharedContractDriftObserver>) {
    if let Ok(mut slot) = GLOBAL_OBSERVER.write() {
        *slot = observer;
    }
}

/// Returns the process-wide fallback observer, if any.
pub fn global_contract_drift_observer() -> Option<SharedContractDriftObserver> {
    GLOBAL_OBSERVER.read().ok().and_then(|g| g.clone())
}

/// Emits a drift signal to the request-scoped observer, else the global one.
///
/// Never panics: observer panics are swallowed. No-op when no observer is set.
pub fn emit_contract_drift(signal: ContractDriftSignal) {
    let observer = REQUEST_SCOPE
        .with(|cell| cell.borrow().observer.clone())
        .or_else(global_contract_drift_observer);
    let Some(observer) = observer else {
        return;
    };
    let _ = catch_unwind(AssertUnwindSafe(|| observer.on_drift(&signal)));
}

/// Convenience: unknown enum value with request-scoped operation when present.
pub(crate) fn emit_unknown_enum(field_path: Option<&'static str>, raw: &str) {
    let operation = REQUEST_SCOPE.with(|cell| cell.borrow().operation);
    emit_contract_drift(ContractDriftSignal::new(
        ContractDriftKind::UnknownEnumValue,
        operation,
        field_path,
        raw,
    ));
}

/// Convenience: off-origin pagination link rejected.
pub(crate) fn emit_off_origin_pagination_link(href_host: &str) {
    let operation = REQUEST_SCOPE.with(|cell| cell.borrow().operation);
    emit_contract_drift(ContractDriftSignal::new(
        ContractDriftKind::OffOriginPaginationLink,
        operation,
        Some("links.next.href"),
        href_host,
    ));
}

fn redact_detail(raw: &str) -> String {
    let trimmed = raw.trim();
    // Never treat Authorization-looking material as safe detail.
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains("bearer ")
        || lower.contains("live_")
        || lower.contains("test_")
        || lower.contains("access_token")
    {
        return "<redacted>".to_string();
    }
    if trimmed.len() <= CONTRACT_DRIFT_DETAIL_MAX_LEN {
        return trimmed.to_string();
    }
    let mut out = trimmed
        .chars()
        .take(CONTRACT_DRIFT_DETAIL_MAX_LEN.saturating_sub(1))
        .collect::<String>();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Mutex, OnceLock};

    fn global_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct CountingObserver {
        count: AtomicUsize,
        last: Mutex<Option<ContractDriftSignal>>,
    }

    impl ContractDriftObserver for CountingObserver {
        fn on_drift(&self, signal: &ContractDriftSignal) {
            self.count.fetch_add(1, Ordering::SeqCst);
            *self.last.lock().unwrap() = Some(signal.clone());
        }
    }

    struct PanickingObserver;

    impl ContractDriftObserver for PanickingObserver {
        fn on_drift(&self, _signal: &ContractDriftSignal) {
            panic!("observer must not poison sdk");
        }
    }

    #[test]
    fn truncates_long_detail() {
        let long = "a".repeat(CONTRACT_DRIFT_DETAIL_MAX_LEN + 40);
        let s = ContractDriftSignal::new(ContractDriftKind::UnknownEnumValue, None, None, long);
        assert!(s.detail_redacted.chars().count() <= CONTRACT_DRIFT_DETAIL_MAX_LEN);
        assert!(s.detail_redacted.ends_with('…'));
    }

    #[test]
    fn redacts_secret_looking_detail() {
        let s = ContractDriftSignal::new(
            ContractDriftKind::UnknownEnumValue,
            None,
            None,
            "Bearer secret-token-value",
        );
        assert_eq!(s.detail_redacted, "<redacted>");
    }

    #[test]
    fn emit_reaches_global_observer() {
        let _lock = global_test_lock().lock().unwrap();
        let obs = Arc::new(CountingObserver {
            count: AtomicUsize::new(0),
            last: Mutex::new(None),
        });
        set_global_contract_drift_observer(Some(obs.clone()));
        emit_contract_drift(ContractDriftSignal::new(
            ContractDriftKind::UnknownEnumValue,
            Some("list_payments"),
            Some("status"),
            "awaiting_unicorn",
        ));
        assert_eq!(obs.count.load(Ordering::SeqCst), 1);
        let last = obs.last.lock().unwrap().clone().unwrap();
        assert_eq!(last.kind, ContractDriftKind::UnknownEnumValue);
        assert_eq!(last.operation, Some("list_payments"));
        assert_eq!(last.detail_redacted, "awaiting_unicorn");
        set_global_contract_drift_observer(None);
    }

    #[test]
    fn panicking_observer_is_isolated() {
        let _lock = global_test_lock().lock().unwrap();
        set_global_contract_drift_observer(Some(Arc::new(PanickingObserver)));
        emit_contract_drift(ContractDriftSignal::new(
            ContractDriftKind::OffOriginPaginationLink,
            None,
            None,
            "evil.example",
        ));
        set_global_contract_drift_observer(None);
    }

    #[test]
    fn request_scope_prefers_client_observer() {
        let _lock = global_test_lock().lock().unwrap();
        let global = Arc::new(CountingObserver {
            count: AtomicUsize::new(0),
            last: Mutex::new(None),
        });
        let local = Arc::new(CountingObserver {
            count: AtomicUsize::new(0),
            last: Mutex::new(None),
        });
        set_global_contract_drift_observer(Some(global.clone()));
        {
            let _guard = ContractDriftScopeGuard::enter("create_payment", Some(local.clone()));
            emit_unknown_enum(Some("PaymentStatus"), "brand_new");
        }
        assert_eq!(local.count.load(Ordering::SeqCst), 1);
        assert_eq!(global.count.load(Ordering::SeqCst), 0);
        set_global_contract_drift_observer(None);
    }
}

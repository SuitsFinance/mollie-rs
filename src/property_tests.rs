//! Lightweight property-style tests for high-value parsers and the retry model.
//!
//! Full cargo-fuzz corpora can be added later; these exercises adversarial
//! inputs without an extra dependency.

use crate::ids::PaymentId;
use crate::money::{AmountValue, Currency, Money};
use crate::pagination::PageCursor;
use crate::transport::{
    compute_backoff, simulate_retry_loop, AttemptEvent, DeliveryOutcome, RetryClass, RetryPolicy,
};
use crate::webhook::WebhookNotification;
use crate::webhook_verify::{compute_mollie_signature_hex, WebhookVerifier};
use std::time::Duration;

#[test]
fn money_parser_rejects_garbage_and_accepts_supported_pairs() {
    let eur = Currency::parse("EUR").expect("EUR");
    for bad in ["", "10", "10.0.0", "abc", "1e2", "-1.00", "10,00"] {
        assert!(
            AmountValue::parse(eur, bad).is_err(),
            "expected reject for {bad:?}"
        );
    }
    for code in Currency::SUPPORTED {
        let money = Money::new(code.code(), "10.00").expect("supported");
        assert_eq!(money.currency().code(), code.code());
    }
    assert!(Currency::parse("ZZZ").is_err());
}

#[test]
fn payment_id_parser_roundtrip_and_rejects() {
    let id = PaymentId::parse("tr_WDqYK6vllg").expect("valid sample shape");
    assert!(id.as_str().starts_with("tr_"));
    for bad in ["", "tr_", "payment_1", "TR_ABC", "tr_ space"] {
        assert!(
            PaymentId::parse(bad).is_err(),
            "expected reject for {bad:?}"
        );
    }
}

#[test]
fn page_cursor_empty_is_none() {
    assert!(PageCursor::new("").is_none());
    assert!(PageCursor::new("abc123").is_some());
}

#[test]
fn classic_webhook_form_parser_property() {
    let n = WebhookNotification::parse_form_urlencoded("id=tr_WDqYK6vllg").expect("form");
    assert_eq!(n.id(), "tr_WDqYK6vllg");
    for raw in ["", "id=", "foo=bar", "%zz", "id=tr_WDqYK6vllg&extra=1"] {
        let _ = WebhookNotification::parse_form_urlencoded(raw);
    }
}

#[test]
fn webhook_signature_parser_never_panics_on_junk() {
    let verifier = WebhookVerifier::new("secret").unwrap();
    let body = br#"{"a":1}"#;
    let long_f = "f".repeat(64);
    let long_g = "g".repeat(64);
    for sig in [
        "",
        " ",
        "not-hex",
        "sha256=",
        "sha256=zz",
        "sha256=abcd",
        long_f.as_str(),
        long_g.as_str(),
    ] {
        let _ = verifier.verify(body, sig);
    }
    let good = compute_mollie_signature_hex(b"secret", body).unwrap();
    verifier.verify(body, &good).unwrap();
}

#[test]
fn retry_after_style_numeric_strings_are_bounded_in_backoff() {
    let policy = RetryPolicy::default_safe();
    for secs in [0u64, 1, 7, 3600] {
        let d = compute_backoff(&policy, 2, Some(Duration::from_secs(secs)));
        assert!(d >= Duration::from_millis(1));
        assert!(d <= policy.max_backoff);
    }
}

/// INV-WRITE-02: financial / idempotent writes without sticky key execute ≤ 1 attempt
/// across mixed failure sequences.
#[test]
fn model_financial_write_without_sticky_at_most_one_attempt() {
    let policy = RetryPolicy::default_safe();
    let sequences: &[&[AttemptEvent]] = &[
        &[AttemptEvent::ConnectFailure, AttemptEvent::Success],
        &[
            AttemptEvent::Timeout,
            AttemptEvent::Timeout,
            AttemptEvent::Success,
        ],
        &[
            AttemptEvent::Status429,
            AttemptEvent::Status503,
            AttemptEvent::Success,
        ],
        &[
            AttemptEvent::Status503,
            AttemptEvent::DeadlineExhausted,
            AttemptEvent::Success,
        ],
        &[
            AttemptEvent::Timeout,
            AttemptEvent::Status429,
            AttemptEvent::Status503,
        ],
    ];
    for events in sequences {
        let sim = simulate_retry_loop(RetryClass::IdempotentWrite, false, &policy, events);
        assert!(
            sim.attempts_started <= 1,
            "expected ≤1 attempt without sticky, got {} for {events:?}",
            sim.attempts_started
        );
    }
    let never = simulate_retry_loop(
        RetryClass::NonRetryableWrite,
        true,
        &policy,
        &[AttemptEvent::Timeout, AttemptEvent::Success],
    );
    assert_eq!(never.attempts_started, 1);
}

/// INV-DEADLINE-01: no attempt begins after total_deadline marker in the schedule.
#[test]
fn model_no_attempt_begins_after_deadline() {
    let policy = RetryPolicy::default_safe();
    let sim = simulate_retry_loop(
        RetryClass::SafeRead,
        false,
        &policy,
        &[
            AttemptEvent::Status503,
            AttemptEvent::DeadlineExhausted,
            AttemptEvent::Success,
            AttemptEvent::Success,
        ],
    );
    assert_eq!(sim.attempts_started, 1);
    assert!(!sim.succeeded);
}

/// INV-DELIV-01: Unknown (timeout) is not collapsed into NotSent; sticky write may retry.
#[test]
fn model_unknown_timeout_retries_only_with_sticky_write() {
    let policy = RetryPolicy::default_safe();
    let without = simulate_retry_loop(
        RetryClass::IdempotentWrite,
        false,
        &policy,
        &[AttemptEvent::Timeout, AttemptEvent::Success],
    );
    assert_eq!(without.attempts_started, 1);
    assert_eq!(without.last_outcome, Some(DeliveryOutcome::Unknown));

    let with_key = simulate_retry_loop(
        RetryClass::IdempotentWrite,
        true,
        &policy,
        &[AttemptEvent::Timeout, AttemptEvent::Success],
    );
    assert_eq!(with_key.attempts_started, 2);
    assert!(with_key.succeeded);
}

#[test]
fn page_cursor_rejects_evil_next_host() {
    assert!(PageCursor::from_list_link("https://evil.example/?from=x").is_none());
    assert!(PageCursor::from_list_link("https://api.mollie.com/v2/x?from=ok").is_some());
}

/// Safe reads may retry connect/429 until success within `max_attempts` (default_safe = 3).
#[test]
fn model_safe_read_retries_transient_then_succeeds() {
    let policy = RetryPolicy::default_safe();
    assert_eq!(policy.max_attempts, 3);
    let sim = simulate_retry_loop(
        RetryClass::SafeRead,
        false,
        &policy,
        &[
            AttemptEvent::ConnectFailure,
            AttemptEvent::Status429,
            AttemptEvent::Success,
            AttemptEvent::Success, // never consumed past max_attempts
        ],
    );
    assert_eq!(sim.attempts_started, 3);
    assert!(sim.succeeded);
    assert_eq!(sim.last_outcome, Some(DeliveryOutcome::Succeeded));
}

/// Non-retryable writes never auto-retry even with sticky key (e.g. OAuth, VoP).
#[test]
fn model_non_retryable_write_never_retries() {
    let policy = RetryPolicy::default_safe();
    for sticky in [false, true] {
        let sim = simulate_retry_loop(
            RetryClass::NonRetryableWrite,
            sticky,
            &policy,
            &[
                AttemptEvent::ConnectFailure,
                AttemptEvent::Timeout,
                AttemptEvent::Status503,
                AttemptEvent::Success,
            ],
        );
        assert_eq!(
            sim.attempts_started, 1,
            "NonRetryableWrite must not retry (sticky={sticky})"
        );
        assert!(!sim.succeeded);
    }
}

/// NotSent (connect) on IdempotentWrite without sticky stays single-shot; with sticky may retry.
#[test]
fn model_not_sent_idempotent_write_sticky_gate() {
    let policy = RetryPolicy::default_safe();
    let bare = simulate_retry_loop(
        RetryClass::IdempotentWrite,
        false,
        &policy,
        &[AttemptEvent::ConnectFailure, AttemptEvent::Success],
    );
    assert_eq!(bare.attempts_started, 1);
    assert_eq!(bare.last_outcome, Some(DeliveryOutcome::NotSent));

    let sticky = simulate_retry_loop(
        RetryClass::IdempotentWrite,
        true,
        &policy,
        &[AttemptEvent::ConnectFailure, AttemptEvent::Success],
    );
    assert_eq!(sticky.attempts_started, 2);
    assert!(sticky.succeeded);
}

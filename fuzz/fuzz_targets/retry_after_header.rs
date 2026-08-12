//! Fuzz Retry-After / backoff computation inputs.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::{compute_backoff, RetryPolicy};
use std::time::Duration;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let attempt = u32::from(data[0]).saturating_add(1).min(32);
    let secs = u64::from(data[1]);
    let policy = RetryPolicy::default_safe();
    let _ = compute_backoff(&policy, attempt, Some(Duration::from_secs(secs)));
    let _ = compute_backoff(&policy, attempt, None);
    // Interpret remaining bytes as a header-like string (parsers elsewhere).
    let s = String::from_utf8_lossy(&data[2..]);
    if let Ok(n) = s.trim().parse::<u64>() {
        let _ = compute_backoff(&policy, attempt, Some(Duration::from_secs(n.min(86_400))));
    }
});

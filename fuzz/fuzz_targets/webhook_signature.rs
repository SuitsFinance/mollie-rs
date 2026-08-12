//! Fuzz Next-gen webhook signature verification (header normalize + HMAC path).
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::{compute_mollie_signature_hex, WebhookVerifier};

fuzz_target!(|data: &[u8]| {
    // Split input: first byte selects body length budget, rest is body+sig bytes.
    if data.is_empty() {
        return;
    }
    let split = (data[0] as usize).saturating_add(1).min(data.len());
    let body = &data[..split.min(data.len())];
    let sig_bytes = if split < data.len() {
        &data[split..]
    } else {
        &[]
    };
    let sig = String::from_utf8_lossy(sig_bytes);

    let Ok(verifier) = WebhookVerifier::new("fuzz-secret") else {
        return;
    };
    let _ = verifier.verify(body, sig.as_ref());
    let _ = verifier.verify_header(body, Some(sig.as_ref()));
    let _ = verifier.verify_header(body, None);

    // Known-good path must not panic either.
    if let Ok(good) = compute_mollie_signature_hex(b"fuzz-secret", body) {
        let _ = verifier.verify(body, &good);
        let _ = verifier.verify(body, &format!("sha256={good}"));
    }
});

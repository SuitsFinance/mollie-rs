//! Fuzz resource payment id parser.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::PaymentId;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = PaymentId::parse(s.as_ref());
});

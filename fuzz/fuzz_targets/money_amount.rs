//! Fuzz money / amount value parsers.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::{AmountValue, Currency, Money};

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = Currency::parse(s.as_ref());
    let _ = Money::new("EUR", s.as_ref());
    if let Ok(eur) = Currency::parse("EUR") {
        let _ = AmountValue::parse(eur, s.as_ref());
    }
    // Two-field split: currency | value
    if let Some((a, b)) = s.split_once('|') {
        let _ = Money::new(a, b);
    }
});

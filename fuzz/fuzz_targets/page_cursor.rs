//! Fuzz pagination cursor helpers.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::PageCursor;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = PageCursor::new(s.as_ref());
    let _ = PageCursor::from_list_link(s.as_ref());
    // Also try as a plausible list next href.
    let href = format!("https://api.mollie.com/v2/payments?from={s}&limit=50");
    let _ = PageCursor::from_list_link(&href);
});

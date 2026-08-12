//! Fuzz classic form-urlencoded webhook parser.
#![no_main]

use libfuzzer_sys::fuzz_target;
use mollie_rs::WebhookNotification;

fuzz_target!(|data: &[u8]| {
    let _ = WebhookNotification::parse_form_urlencoded(data);
});

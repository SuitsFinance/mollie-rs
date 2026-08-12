# locale

## Summary
`Locale` validates Mollie locale identifiers (ISO 15897 `xx_XX`). Mollie documents a fixed **possible values** set; any other well-formed `xx_XX` is still parseable as `Other`.

## Symbol
- Name: `Locale`
- Kind: `enum`
- Owner: `mollie_rs::locale`

## Signature
```rust
pub enum Locale {
    // Possible values (22):
    EnUs, EnGb, NlNl, NlBe, FrFr, FrBe, DeDe, DeAt, DeCh,
    EsEs, CaEs, PtPt, ItIt, NbNo, SvSe, FiFi, DaDk, IsIs,
    HuHu, PlPl, LvLv, LtLt,
    // Extra named / other ISO forms:
    CsCz, DeLu, FrLu, SkSk,
    Other([u8; 5]),
}
```

## Location
- `src/locale.rs`
- ISO notes: `docs/iso/iso-15897.md`

## Inputs
- Named variants / constants such as `Locale::NlNl` / `Locale::NL_NL`.
- `Locale::parse(value)` accepts possible locales and any `xx_XX` (`[a-z]{2}_[A-Z]{2}`).
- `TryFrom<&str>` and `TryFrom<String>` delegate to `Locale::parse`.
- `From<types::LocaleInner>` maps generated OpenAPI locales into the facade.

## Returns
- `as_str()` returns the wire identifier (`"en_US"`, `"nl_NL"`, …).
- `is_possible()` / `is_hosted()` — membership in Mollie's documented set.
- `is_other()` — non-named ISO form.
- `POSSIBLE` / `HOSTED` — the 22 documented values.
- `into_generated()` / `TryFrom` → `types::Locale` when present on the generated enum.
- Method query helpers: `into_list_methods_locale`, `into_list_all_methods_locale`, `into_get_method_locale`.

## Errors
- `Locale::parse` returns `MollieError::InvalidRequest` for non-`xx_XX` strings.
- `into_generated` returns `MollieError::InvalidRequest` when the value is not on `types::LocaleInner`.

## Preconditions
- Possible values: `en_US`, `en_GB`, `nl_NL`, `nl_BE`, `fr_FR`, `fr_BE`, `de_DE`, `de_AT`, `de_CH`, `es_ES`, `ca_ES`, `pt_PT`, `it_IT`, `nb_NO`, `sv_SE`, `fi_FI`, `da_DK`, `is_IS`, `hu_HU`, `pl_PL`, `lv_LV`, `lt_LT`.

## Side Effects
- None.

## Guarantees
- Every entry in `POSSIBLE` maps successfully through `into_generated()`.
- Omitting locale remains valid (`None`).

## Examples
```rust
use mollie_rs::{types::CreatePaymentRequest, Locale, Money};

# fn main() -> Result<(), mollie_rs::MollieError> {
let payment = PaymentRequest {
    amount: Some(Money::new("EUR", "10.00")?.into()),
    description: Some("Order #12345".parse().expect("static")),
    redirect_url: Some("https://example.com/return".to_string()),
    locale: Some(Locale::NL_NL.into_generated()?),
    ..Default::default()
};
assert!(Locale::is_possible_value("en_US"));
# let _ = payment;
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/locale.rs`
- Generated: `types::Locale` / `types::LocaleInner`
- Docs: `docs/iso/iso-15897.md`

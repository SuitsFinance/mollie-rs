# Common data type contracts

The generated OpenAPI models remain the complete wire surface. The facade
adds validation for values with cross-route rules:

| Mollie value | Facade type | Contract |
| --- | --- | --- |
| Resource identifier | `PaymentId`, `CustomerId`, and other typed IDs | Correct resource prefix, allowed characters, non-empty suffix, maximum 32 characters |
| Amount | `Money` | Validated Mollie currency and exact decimal amount |
| Country | `CountryCode` | ISO 3166-1 alpha-2 |
| Date and datetime | `Date`, `DateTime` | ISO 8601 parsing |
| Locale | `Locale` | ISO 15897 `xx_XX` value |
| Phone number | `PhoneNumber` | E.164 format |
| Address | `Address` | Required names, street, city, country, and country-dependent postal code |

## Address

Use `Address` for request construction instead of passing an unchecked
generated address struct:

```rust
use mollie_rs::Address;

let address = Address::new(
    "Floris",
    "Xylex",
    "Main Street 1",
    "Amsterdam",
    "NL",
)?
.with_postal_code("1012AB")?;

let payment_address = address.into_payment_address()?;
# Ok::<(), mollie_rs::MollieError>(())
```

Postal codes may be omitted only for the countries listed in
`POSTAL_CODE_OPTIONAL_COUNTRIES`. Generated `types::Address` and
`types::PaymentAddress` remain available for response deserialization and
native escape-hatch calls.

Business categories, legal entities, registration offices, and other
route-specific enumerations remain generated provider-native strings because
the OpenAPI contract does not expose one shared schema for all of their uses.

# ISO standards used by this SDK

Reference notes for the ISO identifiers Mollie (and this crate) use on the wire.

| Document | Standard | Crate type | Role in Mollie |
| --- | --- | --- | --- |
| [iso-3166-1-alpha-2.md](./iso-3166-1-alpha-2.md) | ISO 3166-1 alpha-2 | [`CountryCode`](../contracts/countryCode.md) | Country codes (`NL`, `DE`, billing/address country) |
| [iso-3166-1-alpha-1.md](./iso-3166-1-alpha-1.md) | Naming note + subdivisions | — | Clarifies alpha-2 vs ISO 3166-2 subdivision codes |
| [iso-4217.md](./iso-4217.md) | ISO 4217 | [`Currency`](../contracts/currency.md) / [`Money`](../contracts/money.md) | Currency codes (`EUR`, `USD`, …) |
| [iso-8601.md](./iso-8601.md) | ISO 8601 | [`DateTime`](../contracts/dateTime.md) / [`Date`](../contracts/dateTime.md) | Datetimes (`createdAt`, `expiresAt`, …) and calendar dates |
| [iso-15897.md](./iso-15897.md) | ISO 15897 (locale form) | [`Locale`](../contracts/locale.md) | Locales (`en_US`, `nl_NL`, …) |
| [E.164 phone numbers](../e.164.md) | ITU-T E.164 | [`PhoneNumber`](../contracts/phoneNumber.md) | Address / profile `phone` (`+31208202070`) |

These docs are **SDK orientation**, not the full legal text of each standard.

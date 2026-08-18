# Tier-S workflow → example matrix (EX-001)

Machine source: [`docs/registries/tier-s-workflow-examples.yaml`](../registries/tier-s-workflow-examples.yaml).
Gate: `python scripts/check_workflow_examples.py` (also CI contracts job).

Examples under `examples/*.rs` are **generated** (`scripts/route_examples.py`); do not hand-edit them.
This matrix asserts required money-path / Tier-S workflows keep at least one compile-checked example
(CI also runs `cargo check --examples --all-features`).

| Workflow | Title | Example crate(s) | Guide(s) |
| --- | --- | --- | --- |
| `payments.create` | Create payment | `create_payment.rs` | `docs/guides/payments.md` |
| `payments.get` | Get payment | `get_payment.rs` | `docs/guides/payments.md` |
| `payments.list` | List payments (pagination entry) | `list_payments.rs` | `docs/guides/pagination.md` |
| `payments.update` | Update payment | `update_payment.rs` | — |
| `payments.cancel` | Cancel payment | `cancel_payment.rs` | — |
| `refunds.create` | Create refund | `create_refund.rs` | `docs/guides/refunds.md` |
| `refunds.get` | Get refund | `get_refund.rs` | — |
| `refunds.list` | List refunds | `list_refunds.rs`, `list_all_refunds.rs` | — |
| `payouts.create` | Create payout | `create_payout.rs` | `docs/guides/payouts-and-transfers.md` |
| `payouts.get` | Get payout | `get_payout.rs` | — |
| `payouts.list` | List payouts | `list_payouts.rs` | — |
| `payouts.cancel` | Cancel payout | `cancel_payout.rs` | — |
| `transfers.create` | Create transfer | `create_transfer.rs` | `docs/guides/payouts-and-transfers.md` |
| `transfers.get` | Get transfer | `get_transfer.rs` | — |
| `connect_balance_transfers.create` | Create connect balance transfer | `create_connect_balance_transfer.rs` | — |
| `connect_balance_transfers.list` | List connect balance transfers | `list_connect_balance_transfers.rs` | — |
| `payment_links.create` | Create payment link | `create_payment_link.rs` | — |
| `payment_links.list` | List payment links | `list_payment_links.rs` | — |
| `mandates.create` | Create mandate | `create_mandate.rs` | — |
| `mandates.list` | List mandates | `list_mandates.rs` | — |
| `mandates.revoke` | Revoke mandate | `revoke_mandate.rs` | — |
| `subscriptions.create` | Create subscription | `create_subscription.rs` | — |
| `subscriptions.list` | List subscriptions | `list_subscriptions.rs`, `list_all_subscriptions.rs` | — |
| `subscriptions.cancel` | Cancel subscription | `cancel_subscription.rs` | — |
| `captures.create` | Create capture | `create_capture.rs` | — |
| `captures.list` | List captures | `list_captures.rs` | — |
| `terminals.get` | Get terminal | `get_terminal.rs` | — |
| `terminals.list` | List terminals | `list_terminals.rs` | — |
| `terminals.pairing` | Terminal pairing codes | `terminals_request_pairing_code.rs`, `terminals_list_pairing_codes.rs` | — |
| `oauth.tokens` | OAuth token lifecycle | `oauth_generate_tokens.rs`, `oauth_revoke_tokens.rs` | `docs/guides/oauth-connect.md` |
| `webhooks.create` | Create webhook | `create_webhook.rs` | `docs/guides/handle-signed-webhook.md` |
| `webhooks.get` | Get webhook | `get_webhook.rs` | — |
| `webhooks.list` | List webhooks | `list_webhooks.rs` | — |
| `webhooks.test` | Test webhook | `test_webhook.rs` | — |
| `sessions.create` | Create session | `create_session.rs` | — |
| `sessions.get` | Get session | `get_session.rs` | — |
| `unmatched_credit_transfers.list` | List unmatched credit transfers | `list_unmatched_credit_transfers.rs` | — |
| `unmatched_credit_transfers.get` | Get unmatched credit transfer | `get_unmatched_credit_transfer.rs` | — |
| `verify_payee` | Verify payee | `verify_payee.rs` | — |

## Residual notes

- Stream APIs (`stream_pages` / `stream_items`) are exercised in unit tests and documented in
  [`pagination.md`](../guides/pagination.md); list examples cover the page entry point.
- Webhook **signature verification** is covered by `docs/guides/handle-signed-webhook.md` and
  library tests (`VerifiedWebhook`); route examples cover webhook CRUD/test endpoints.
- Multi-merchant / OAuth app helpers: see `docs/guides/oauth-connect.md` and `multi-merchant.md`.

# High-risk operation inventory

**HEAD:** `e3358d2e49cb065d690deea8b43cdf2c9ed93a8a`  
**SSOT capability table:** `src/route_capabilities.rs`  
**CI denominator script:** `scripts/check_dangerous_profile_drift.py`  
**Profile view:** `src/operation_safety.rs` (`OperationSafetyProfile = RouteCapability`)

---

## 1. Protection rubric (mission)

An operation is **fully protected** only if all hold:

1. Explicit operation safety profile row  
2. Auth class known (derived OK)  
3. Mutation class known  
4. Retry class known  
5. Idempotency semantics known  
6. Delivery ambiguity defined (`DeliveryOutcome`)  
7. Transport enforces rules  
8. Errors expose delivery context  
9. Secret-leak tests cover related secret types  
10. Negative tests reject unsafe retry/behavior  
11. Tier-S facade when it materially improves correctness  

---

## 2. CI high-risk write set (current denominator = 16)

From `HIGH_RISK_WRITES` (blocking):

| operation_id | Tier-S | ValidatedFacade | Notes |
| --- | --- | --- | --- |
| `create_payment` | Yes `payments` | Yes | |
| `create_refund` | Yes `refunds` | Yes | |
| `create_capture` | Yes `captures` | Yes | |
| `create_subscription` | Yes `subscriptions` | Yes | |
| `create_payout` | Yes `payouts` | Yes | |
| `cancel_payout` | Yes `payouts` | Yes | |
| `create_transfer` | Yes `transfers` | Yes | signing |
| `verify_payee` | Yes `verify_payee` | Yes | |
| `oauth_generate_tokens` | Yes `oauth` | Yes | NonRetryable |
| `oauth_revoke_tokens` | Yes `oauth` | Yes | NonRetryable |
| `payment_create_route` | via payments/routes | Yes | |
| `create_session` | Yes `sessions` | Yes | |
| `terminals_request_pairing_code` | Yes `terminals` | Yes | |
| `terminals_revoke_pairing_code` | Yes `terminals` | Yes | |
| `match_unmatched_credit_transfer` | Yes UCT | Yes | |
| `return_unmatched_credit_transfer` | Yes UCT | Yes | |

**CI report (this freeze):** `high-risk writes checked: 16` — PASS.

---

## 3. Mission seed set — gap analysis

Mission listed additional financial/credential mutations. Classification vs CI set:

| operation_id | In CI HIGH_RISK? | Tier-S | Risk class | Action |
| --- | --- | --- | --- | --- |
| `cancel_payment` | **No** | payments cancel path | financial / cancellation | **Add** to denominator + proofs |
| `cancel_refund` | **No** | refunds | cancellation | **Add** |
| `create_mandate` | No (but ValidatedFacade) | mandates | financial/mandate | Align CI set with ValidatedFacade |
| `create_payment_link` | No (ValidatedFacade) | payment_links | financial | Align CI set |
| `create_customer_payment` | **No** | payments? | financial | Classify + facade path |
| `create_connect_balance_transfer` | **No** | **No** | merchant-scoped financial | **P0/P1** Tier-S + profile |
| `cancel_subscription` | No | subscriptions | cancellation | Add if auto-retry risk |
| Webhook verify path | N/A (inbound) | webhooks | credential/HMAC | Keep WHK suite separate |

**Finding HR-001:** Denominator under-counts mission financial surface. 1.0 metric must freeze an explicit machine-readable set (extend `HIGH_RISK_WRITES` or generate from `MutationClass` + allowlist), then report:

```text
Fully protected: N / D
```

Do not claim 100% on the 16-set alone if Connect/cancel paths remain thinner.

---

## 4. ValidatedFacade ops (18) not all in HIGH_RISK

ValidatedFacade today:

```text
oauth_generate_tokens, oauth_revoke_tokens, create_payment,
match_unmatched_credit_transfer, return_unmatched_credit_transfer,
create_session, create_refund, create_capture, create_payment_link,
terminals_request_pairing_code, terminals_revoke_pairing_code,
payment_create_route, create_mandate, create_subscription,
create_transfer, verify_payee, create_payout, cancel_payout
```

**Gap:** `create_mandate` + `create_payment_link` are ValidatedFacade but outside `HIGH_RISK_WRITES`.  
**Gap:** Connect create is neither.

---

## 5. Coverage dimensions (template for generator)

For each high-risk op track:

| Dim | Source |
| --- | --- |
| profile row | capabilities |
| auth_class | `operation_safety` |
| mutation_class | derived |
| retry_class | capabilities |
| idempotency_class | derived |
| delivery_aware | transport + error |
| Tier-S | `src/domain/*` |
| validated request | ValidatedFacade / builders |
| secret-leak | `secret_leak_tests` |
| negative retry | property_tests / http_contract |
| status | Open / Partial / Full |

**Target artifact:** `scripts/report_high_risk_coverage.py` → `docs/registries/high-risk-coverage.md` + JSON (Phase program).

---

## 6. Interim score (evidence-bound, not flattery)

| Metric | Value | Note |
| --- | ---: | --- |
| Contract ops | 124/124 | Closed |
| Safety profiles | 124/124 | Closed |
| CI high-risk profile checks | 16/16 | Closed for **that** set |
| Mission-expanded denominator | **UNFROZEN** | HR-001 |
| Connect financial Tier-S | 0 | HR-002 |
| Retry-After HTTP-date | fail | PAY-004 |
| Production guides | 3/12 | DOC-GUIDE-01 |

**Do not publish “23/23 fully protected” until denominator freeze + generator exist.**

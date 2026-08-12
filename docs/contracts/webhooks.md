# Webhook contracts

## Classic callback handling

Mollie classic webhook requests are `POST` requests with an
`application/x-www-form-urlencoded` body containing only the updated resource
identifier:

```text
id=tr_d0b0E3EA3v
```

Parse that body with `WebhookNotification::parse_form_urlencoded`. The
notification does not contain a trusted payment status. After acknowledging
the request, refetch the resource through the authenticated Mollie client and
compare the returned state with the application record.

```rust
use mollie_rs::WebhookNotification;

let notification = WebhookNotification::parse_form_urlencoded("id=tr_d0b0E3EA3v")?;
let resource_id = notification.id();
// Parse `resource_id` as the expected typed ID and refetch it from Mollie.
# Ok::<(), mollie_rs::MollieError>(())
```

Applications should return `200 OK` for unknown IDs to avoid leaking whether a
resource exists. A webhook handler should respond within 15 seconds. Mollie
retries non-`200` responses with increasing intervals, up to ten attempts.

## Webhook destination URLs

`WebhookUrl::parse` accepts absolute `http` and `https` URLs and rejects
`localhost` and loopback destinations. Mollie must be able to reach the URL;
for local development, expose the handler through a public tunnel such as
ngrok. Redirect webhook endpoints with `307` or `308` so the `POST` method and
form body are retained.

Payment and subscription builders validate webhook URLs through this type.

## Next-gen webhooks

Next-gen Webhooks are a beta API with a separate event and delivery contract.
They are not decoded into `WebhookNotification`; use the generated native
route surface and preserve the provider payload until the beta contract is
stable.

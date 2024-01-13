# Endpoint status

This document categorizes the APIs as they pertain to this crate.

Last synced: v0.1.0.

# Implemented

These API endpoints have been implemented.

- [Notify API](https://docs.alchemy.com/reference/notify-api-quickstart):
  - `GET    /api/team-webhooks` `notify/team_webhooks.rs`
  - `GET    /api/webhook-addresses` `notify/webhook_addresses/get.rs`
  - `POST   /api/create-webhook` `notify/create_webhooks.rs`
  - `PATCH  /api/update-webhook-addresses` `notify/webhook_addresses/update.rs`
  - `PUT    /api/update-webhook-addresses` `notify/webhook_addresses/replace.rs`
  - `PUT    /api/update-webhook` `notify/update_webhook.rs`
  - `PATCH  /api/update-webhook-nft-filters` `notify/nft_filters/update.rs`
  - `PATCH  /api/update-webhook-nft-metadata-filters`
    `notify/nft_filters/update_metadata.rs`
  - `GET    /api/webhook-nft-filters` `notify/nft_filters/get.rs`
  - `DELETE /api/delete-webhook` `notify/delete_webhook.rs`

# Todo

This section contains the list of API endpoints which are not yet implemented in
this crate. Contributions welcome!

- [NFT API](https://docs.alchemy.com/reference/nft-api-quickstart).
- [Transfers API](https://docs.alchemy.com/reference/transfers-api-quickstart).
- [Transaction Receipts API](https://docs.alchemy.com/reference/transaction-receipts-quickstart).
- [Token API](https://docs.alchemy.com/reference/token-api-quickstart).
- [Trace API](https://docs.alchemy.com/reference/trace-api-quickstart).
- [Subscription API](https://docs.alchemy.com/reference/subscription-api).

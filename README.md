# alchemy-api

[![Crates.io](https://img.shields.io/crates/v/alchemy-api.svg)](https://crates.io/crates/alchemy-api)
[![Documentation](https://docs.rs/alchemy-api/badge.svg)](https://docs.rs/alchemy-api/)
[![codecov](https://codecov.io/gh/phnaharris/alchemy-api-rs/graph/badge.svg?token=AWZJZFZCMH)](https://codecov.io/gh/phnaharris/alchemy-api-rs)
[![Build Status](https://github.com/phnaharris/alchemy-api-rs/actions/workflows/main.yml/badge.svg)](https://github.com/phnaharris/alchemy-api-rs/actions/workflows/main.yml)

A high-level binding for [`Alchemy Enhanced APIs`], written in Rust.

Alchemy provides a suite of web3 APIs that dramatically simplify and optimize common request
patterns to make your life as a developer easier.

Access the blockchain like never before with Alchemy's continually expanding Enhanced API
suite, and web3 developer tools! Query NFTs by the user, trace transactions, get real-time
notifications in your dApp, debug smart contracts faster, and do more with Alchemy's supported
endpoints.

This crate uses the [reqwest] crate for a convenient, higher-level HTTP Client, for request and
response, to and from Alchemy, and [serde] for serialize and deserialize from and to
appropriate data format.

## Examples

Let's start out creating an [`AddressActivity`] webhook.

```rust
use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{CreateWebhook, WebhookResponse, WebhookType},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Create a simple endpoint.
    let endpoint = CreateWebhook::builder()
        .webhook_url(std::env::var("WEBHOOK_URL")?)
        .webhook_type(WebhookType::AddressActivity)
        .addresses(vec![std::env::var("ADDRESS")?.parse()?])
        .build()?;
    println!("{:?}", endpoint);

    // Call the endpoint. The return type decides how to represent the value.
    let webhook: WebhookResponse = endpoint.query(&client).await?;
    println!("webhook: {:?}", webhook);

    anyhow::Ok(())
}
```

For more examples, take a look at the `examples/` directory.

[`Alchemy Enhanced APIs`]: https://docs.alchemy.com/reference/enhanced-apis-overview
[reqwest]: https://crates.io/crates/reqwest
[serde]: https://crates.io/crates/serde
[`AddressActivity`]: self::api::notify::WebhookType#AddressActivity

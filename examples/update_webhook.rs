use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{UpdateWebhook, WebhookResponse},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Update a simple endpoint.
    let endpoint = UpdateWebhook::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        .is_active(false)
        .build()?;
    println!("{:?}", endpoint);

    // Call the endpoint. The return type decides how to represent the value.
    let webhook: WebhookResponse = endpoint.query(&client).await?;
    println!("webhook: {:?}", webhook);

    anyhow::Ok(())
}

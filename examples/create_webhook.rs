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

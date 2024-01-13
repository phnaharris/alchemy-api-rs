use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{GetAllWebhooks, GetAllWebhooksResponse},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Create a simple endpoint.
    let endpoint = GetAllWebhooks;
    // Call the endpoint. The return type decides how to represent the value.
    let webhooks: GetAllWebhooksResponse = endpoint.query(&client).await?;
    println!("webhooks: {:?}", webhooks);

    anyhow::Ok(())
}

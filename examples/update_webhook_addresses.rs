use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{EmptyResponse, UpdateWebhookAddresses},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Create a simple endpoint.
    let endpoint = UpdateWebhookAddresses::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        // .addresses_to_add(vec![std::env::var("ADDRESS")?.parse()?])
        .addresses_to_remove(vec![std::env::var("ADDRESS")?.parse()?])
        .build()?;

    // Call the endpoint. The return type decides how to represent the value.
    let resp: EmptyResponse = endpoint.query(&client).await?;
    println!("update response: {:?}", resp);

    anyhow::Ok(())
}

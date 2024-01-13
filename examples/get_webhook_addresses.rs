use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{GetWebhookAddresses, GetWebhookAddressesResponse},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Create a simple endpoint.
    let endpoint = GetWebhookAddresses::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        .build()?;
    // Call the endpoint. The return type decides how to represent the value.
    let addresses: GetWebhookAddressesResponse = endpoint.query(&client).await?;
    println!("webhook addresses: {:?}", addresses);

    anyhow::Ok(())
}

use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{GetWebhookNftFilters, GetWebhookNftFiltersResponse},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Create a simple endpoint.
    let endpoint = GetWebhookNftFilters::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        .limit(1u64)
        .after(2.to_string())
        .build()?;
    println!("{:?}", endpoint);

    // Call the endpoint. The return type decides how to represent the value.
    let filters: GetWebhookNftFiltersResponse = endpoint.query(&client).await?;
    println!("filters: {:?}", filters);

    anyhow::Ok(())
}

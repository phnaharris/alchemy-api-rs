use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{DeleteWebhook, EmptyResponse},
    cores::query::Query,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    // Delete a simple endpoint.
    let endpoint = DeleteWebhook::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        .build()?;
    println!("{:?}", endpoint);

    // Call the endpoint. The return type decides how to represent the value.
    let resp: EmptyResponse = endpoint.query(&client).await?;
    println!("response: {:?}", resp);

    anyhow::Ok(())
}

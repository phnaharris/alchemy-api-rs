use alchemy_api::{
    alchemy::Alchemy,
    api::notify::{EmptyResponse, UpdateWebhookNftFilters},
    cores::query::Query,
    types::common::NftFilter,
};
use ethers_core::types::Address;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Alchemy::new(&std::env::var("ALCHEMY_TOKEN")?);

    let nft_filter = NftFilter::builder()
        .contract_address(std::env::var("NFT_CONTRACT_ADDRESS")?.parse::<Address>()?)
        // .token_id(std::env::var("NFT_TOKEN_ID")?.parse::<usize>()?)
        .build()?;

    // Create a simple endpoint.
    let endpoint = UpdateWebhookNftFilters::builder()
        .webhook_id(std::env::var("WEBHOOK_ID")?)
        .nft_filters_to_add(vec![nft_filter])
        // .nft_filters_to_remove(vec![nft_filter])
        .build()?;

    // Call the endpoint. The return type decides how to represent the value.
    let resp: EmptyResponse = endpoint.query(&client).await?;
    println!("ok response {:?}", resp);

    anyhow::Ok(())
}

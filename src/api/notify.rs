use crate::types::chain::AlchemyChain;
use ethers_core::types::Address;
use serde::{Deserialize, Serialize};

mod create_webhook;
mod delete_webhook;
mod nft_filters;
mod team_webhooks;
mod update_webhook;
mod webhook_addresses;

pub use create_webhook::CreateWebhook;
pub use delete_webhook::DeleteWebhook;
pub use nft_filters::{
    GetWebhookNftFilters, GetWebhookNftFiltersResponse, UpdateWebhookNftFilters,
    UpdateWebhookNftMetadataFilters,
};
pub use team_webhooks::{GetAllWebhooks, GetAllWebhooksResponse};
pub use update_webhook::UpdateWebhook;
pub use webhook_addresses::{
    GetWebhookAddresses, GetWebhookAddressesResponse, ReplaceWebhookAddresses,
    UpdateWebhookAddresses,
};

/// Webhook information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    /// Unique ID for given webhook.
    id: String,
    /// Network of webhook.
    network: AlchemyChain,
    /// Type of webhook, `null` if test webhook.
    webhook_type: Option<WebhookType>,
    /// URL endpoint where webhook is sent.
    webhook_url: String,
    /// `true` if webhook is active, `false` if not active.
    is_active: bool,
    /// Timestamp webhook was created.
    time_created: u64,
    /// List of addresses being tracked, `null` if not address activity webhook.
    addresses: Option<Vec<Address>>,
    /// Webhook version (`v1` or `v2`).
    version: WebhookVersion,
    /// Signing key for given webhook.
    signing_key: String,
}

/// Returns webhook creation data/updated webhook status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    data: Webhook,
}

impl WebhookResponse {
    /// List of webhooks for your team.
    pub fn data(&self) -> Webhook {
        self.data.clone()
    }
}

/// Webhook version (v1 or v2)
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookType {
    /// Custom Webhooks allows you to track any smart contract or marketplace activity, monitor any
    /// contract creation, or any other on-chain interaction. This gives you infinite data access
    /// with precise filter controls to get the blockchain data you need.
    CustomWebhooks,
    /// The Mined Transaction webhook notifies your app when a transaction sent through your app
    /// (using your API key) gets mined. This is useful for you to further notify the users of your
    /// app about the status of the transaction.
    MinedTransaction,
    /// The Dropped Transaction webhook notifies your app when a transaction sent through your app
    /// (using your API key) gets dropped. This is useful for you to further notify the users of
    /// your app about the status of the transaction.
    DroppedTransaction,
    /// Alchemy's Address Activity webhook tracks all ETH, ERC20, ERC721 and ERC1155 transfers.
    /// This provides your app with real-time state changes when an address sends/receives tokens
    /// or ETH. You can specify the addresses for which you want to track this activity. A maximum
    /// of 50,000 addresses can be added to a single webhook.
    #[default]
    AddressActivity,
    /// The NFT Activity webhook allows you to track ERC721 and ERC1155 token contracts for NFTs.
    /// This provides your app with real-time state changes when an NFT is transferred between
    /// addresses.
    NftActivity,
    /// The NFT Metadata Updates webhook allows you to track metadata updates for ERC721 and
    /// ERC1155 token contracts for Ethereum and Polygon NFTs. This notifies your app when the
    /// metadata for an NFT is updated.
    NftMetadataUpdate,
}

/// Webhook version (v1 or v2)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WebhookVersion {
    /// V1 Webhook.
    V1,
    /// V2 Webhook.
    V2,
}

/// Returns empty object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

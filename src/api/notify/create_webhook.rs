use super::WebhookType;
use crate::{
    cores::endpoint::Endpoint,
    types::{chain::AlchemyChain, common::NftFilter},
};
use derive_builder::Builder;
use ethers_core::types::Address;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// This endpoint allows you to create a webhook.
#[serde_as]
#[skip_serializing_none]
#[derive(Builder, Default, Debug, Clone, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct CreateWebhook {
    /// Network of webhook.
    #[builder(setter(skip))]
    network: AlchemyChain,
    /// Type of webhook.
    #[builder(setter)]
    webhook_type: WebhookType,
    /// URL where requests are sent.
    #[builder(setter)]
    webhook_url: String,
    /// Required for mined and dropped webhooks, optional for address activity or custom webhooks.
    #[builder(default)]
    app_id: Option<String>,
    /// List of addresses you want to track. Required for address activity webhooks only.
    #[builder(default)]
    addresses: Vec<Address>,
    /// List of nft filter objects to track transfer activity for.
    #[builder(default)]
    nft_filters: Option<Vec<NftFilter>>,
    /// List of nft metadata filter objects to track metadata updates for.
    #[builder(default)]
    nft_metadata_filters: Option<Vec<NftFilter>>,
}

impl CreateWebhook {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateWebhookBuilder {
        CreateWebhookBuilder::default()
    }
}

impl CreateWebhookBuilder {
    /// Pre-Build Validation.
    fn validate(&self) -> Result<(), String> {
        // Alchemy rules:
        // * `app_id`: required for mined and dropped webhooks, optional for address activity or
        // custom webhooks.
        // * `addresses`: required for address activity webhooks only.
        if let Some(whtype) = self.webhook_type {
            match whtype {
                WebhookType::MinedTransaction | WebhookType::DroppedTransaction
                    if self.app_id.is_none() =>
                {
                    Err("app_id is required for mined and dropped webhooks".to_string())
                }
                WebhookType::AddressActivity
                    if self.addresses.is_none() || self.addresses.as_ref().unwrap().is_empty() =>
                {
                    Err("addresses is required for address activity webhooks".to_string())
                }
                _ => Ok(()),
            }
        } else {
            Err("webhook_type is required".to_string())
        }
    }
}

impl Endpoint for CreateWebhook {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/create-webhook".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

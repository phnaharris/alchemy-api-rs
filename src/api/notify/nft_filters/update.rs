use crate::cores::endpoint::Endpoint;
use crate::types::common::NftFilter;
use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

/// Add and remove webhook NFT filters.
#[derive(Default, Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(default, setter(into), build_fn(validate = "Self::validate"))]
pub struct UpdateWebhookNftFilters {
    /// ID of the address activity webhook.
    webhook_id: String,

    /// List of nft filters to add, use [] if none.
    nft_filters_to_add: Vec<NftFilter>,

    /// List of nft filters to remove, use [] if none.
    nft_filters_to_remove: Vec<NftFilter>,
}

impl UpdateWebhookNftFilters {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateWebhookNftFiltersBuilder {
        UpdateWebhookNftFiltersBuilder::default()
    }
}

impl UpdateWebhookNftFiltersBuilder {
    /// Pre-Build Validation.
    fn validate(&self) -> Result<(), String> {
        // Return Err if neither `addresses_to_add` and `addresses_to_remove` are empty.
        match (&self.nft_filters_to_add, &self.nft_filters_to_remove) {
            (Some(adds), Some(removes)) if !adds.is_empty() || !removes.is_empty() => Ok(()),
            (Some(adds), _) if !adds.is_empty() => Ok(()),
            (_, Some(removes)) if !removes.is_empty() => Ok(()),
            _ => Err("you must provide at least one filter".to_string()),
        }
    }
}

impl Endpoint for UpdateWebhookNftFilters {
    fn method(&self) -> http::Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/update-webhook-nft-filters".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

use crate::cores::endpoint::Endpoint;
use derive_builder::Builder;
use ethers_core::types::Address;
use http::Method;
use serde::{Deserialize, Serialize};

/// Add or remove addresses from a specific webhook.
///
/// This webhook endpoint is idempotent, meaning that identical requests can be made once or
/// several times in a row with the same effect.
#[derive(Default, Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(default, setter(into), build_fn(validate = "Self::validate"))]
pub struct UpdateWebhookAddresses {
    /// ID of the address activity webhook.
    #[builder(setter)]
    webhook_id: String,
    /// List of addresses to add, use [] if none.
    addresses_to_add: Vec<Address>,
    /// List of addresses to remove, use [] if none.
    addresses_to_remove: Vec<Address>,
}

impl UpdateWebhookAddresses {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateWebhookAddressesBuilder {
        UpdateWebhookAddressesBuilder::default()
    }
}

impl UpdateWebhookAddressesBuilder {
    /// Pre-Build Validation.
    fn validate(&self) -> Result<(), String> {
        // Return Err if neither `addresses_to_add` and `addresses_to_remove` are empty.
        match (&self.addresses_to_add, &self.addresses_to_remove) {
            (Some(adds), Some(removes)) if !adds.is_empty() || !removes.is_empty() => Ok(()),
            (Some(adds), _) if !adds.is_empty() => Ok(()),
            (_, Some(removes)) if !removes.is_empty() => Ok(()),
            _ => Err("you must provide at least one address".to_string()),
        }
    }
}

impl Endpoint for UpdateWebhookAddresses {
    fn method(&self) -> http::Method {
        Method::PATCH
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/update-webhook-addresses".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

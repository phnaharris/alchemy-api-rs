use crate::cores::endpoint::Endpoint;
use derive_builder::Builder;
use ethers_core::types::Address;
use http::Method;
use serde::{Deserialize, Serialize};

/// Replace entire list of addresses tracked in a given webhook.
#[derive(Default, Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct ReplaceWebhookAddresses {
    /// ID of the address activity webhook.
    #[builder(setter)]
    webhook_id: String,
    /// New list of addresses to track. This replaces any existing addresses.
    addresses: Vec<Address>,
}

impl ReplaceWebhookAddresses {
    /// Create a builder for the endpoint.
    pub fn builder() -> ReplaceWebhookAddressesBuilder {
        ReplaceWebhookAddressesBuilder::default()
    }
}

impl Endpoint for ReplaceWebhookAddresses {
    fn method(&self) -> http::Method {
        Method::PUT
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/update-webhook-addresses".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

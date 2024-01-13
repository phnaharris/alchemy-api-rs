use crate::{
    cores::{endpoint::Endpoint, params::QueryParams},
    types::common::PaginationObject,
};
use derive_builder::Builder;
use ethers_core::types::Address;
use http::Method;
use serde::{Deserialize, Serialize};

/// Paginated endpoint to list all of the addresses a given `Address Activity` webhook is
/// subscribed to.
#[derive(Default, Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct GetWebhookAddresses {
    /// ID of the address activity webhook.
    #[builder(setter)]
    webhook_id: String,
    /// Number of items per page.
    #[builder(default)]
    limit: Option<u64>,
    /// Page cursor for the next page.
    #[builder(default)]
    after: Option<String>,
}

impl GetWebhookAddresses {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetWebhookAddressesBuilder {
        GetWebhookAddressesBuilder::default()
    }
}

impl Endpoint for GetWebhookAddresses {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/webhook-addresses".into()
    }

    fn parameters(&self) -> crate::cores::params::QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("webhook_id", self.webhook_id.clone());
        params.push_opt("limit", self.limit);
        params.push_opt("after", self.after.clone());
        params
    }
}

/// List of addresses and pagination info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookAddressesResponse {
    data: Vec<Address>,
    pagination: PaginationObject,
}

impl GetWebhookAddressesResponse {
    /// List of addresses associated with given webhook.
    pub fn data(&self) -> Vec<Address> {
        self.data.clone()
    }

    /// Information for pagination.
    pub fn pagination(&self) -> PaginationObject {
        self.pagination.clone()
    }
}

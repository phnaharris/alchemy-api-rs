use crate::{
    cores::{endpoint::Endpoint, params::QueryParams},
    types::common::{NftFilter, PaginationObject},
};
use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

/// Paginated endpoint to list all of the NFT filter objects a given webhook is subscribed to.
#[derive(Default, Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct GetWebhookNftFilters {
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

impl GetWebhookNftFilters {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetWebhookNftFiltersBuilder {
        GetWebhookNftFiltersBuilder::default()
    }
}

impl Endpoint for GetWebhookNftFilters {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/webhook-nft-filters".into()
    }

    fn parameters(&self) -> crate::cores::params::QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("webhook_id", self.webhook_id.clone());
        params.push_opt("limit", self.limit);
        params.push_opt("after", self.after.clone());
        params
    }
}

/// Returns a list of nft filter objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookNftFiltersResponse {
    data: Vec<NftFilter>,
    pagination: PaginationObject,
}

impl GetWebhookNftFiltersResponse {
    /// List of nft filter objects.
    pub fn data(&self) -> Vec<NftFilter> {
        self.data.clone()
    }

    /// Information for pagination.
    pub fn pagination(&self) -> PaginationObject {
        self.pagination.clone()
    }
}

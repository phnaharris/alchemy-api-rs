use crate::cores::{endpoint::Endpoint, params::QueryParams};
use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

/// Allows you to delete a webhook.
#[derive(Builder, Default, Debug, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct DeleteWebhook {
    /// ID of the address activity webhook.
    webhook_id: String,
}

impl DeleteWebhook {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteWebhookBuilder {
        DeleteWebhookBuilder::default()
    }
}

impl Endpoint for DeleteWebhook {
    fn method(&self) -> http::Method {
        Method::DELETE
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/delete-webhook".into()
    }

    fn parameters(&self) -> crate::cores::params::QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("webhook_id", self.webhook_id.clone());
        params
    }
}

use crate::cores::endpoint::Endpoint;
use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Allows you to set status of webhooks to active or inactive.
#[serde_as]
#[skip_serializing_none]
#[derive(Builder, Default, Debug, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct UpdateWebhook {
    /// ID of the address activity webhook.
    webhook_id: String,
    /// true - set webhook to active state
    /// false - set webhook to inactive state
    is_active: bool,
}

impl UpdateWebhook {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateWebhookBuilder {
        UpdateWebhookBuilder::default()
    }
}

impl Endpoint for UpdateWebhook {
    fn method(&self) -> http::Method {
        Method::PUT
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/update-webhook".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

use super::Webhook;
use crate::cores::endpoint::Endpoint;
use http::Method;
use serde::{Deserialize, Serialize};

/// This endpoint allows you to get all webhooks on your team.
#[derive(Default, Debug, Clone)]
pub struct GetAllWebhooks;

impl Endpoint for GetAllWebhooks {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api/team-webhooks".into()
    }
}

/// List of webhook objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllWebhooksResponse {
    data: Vec<Webhook>,
}

impl GetAllWebhooksResponse {
    /// List of nft filter objects.
    pub fn data(&self) -> Vec<Webhook> {
        self.data.clone()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationQueryParams {
    #[serde(rename = "hub.mode")]
    pub hub_mode: String,

    #[serde(rename = "hub.challenge")]
    pub hub_challenge: String,

    #[serde(rename = "hub.verify_token")]
    pub hub_verify_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationResponse {
    #[serde(rename = "hub.challenge")]
    pub hub_challenge: String,
}

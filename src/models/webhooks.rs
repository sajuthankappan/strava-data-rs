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

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookEventData {
    pub object_type: WebhookEventDataObjectType,
    pub object_id: i64,
    pub aspect_type: WebhookEventDataAspectType,
    pub updates: WebhookEventDataUpdates,
    pub owner_id: i32,
    pub subscription_id: i64,
    pub event_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookEventDataUpdates {
    pub authorized: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhookEventDataObjectType {
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "athlete")]
    Athlete,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhookEventDataAspectType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

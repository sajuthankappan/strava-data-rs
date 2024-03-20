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
    object_type: WebhookEventDataObjectType,
    object_id: i64,
    aspect_type: WebhookEventDataAspectType,
    updates: WebhookEventDataUpdates,
    owner_id: i32,
    subscription_id: i64,
    event_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookEventDataUpdates {
    authorized: Option<String>,
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

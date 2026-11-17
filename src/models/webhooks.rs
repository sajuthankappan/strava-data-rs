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
    pub owner_id: i64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_64_bit_ids() {
        let event: WebhookEventData = serde_json::from_str(
            r#"{
                "object_type": "activity",
                "object_id": 9007199254740993,
                "aspect_type": "create",
                "updates": {},
                "owner_id": 9007199254740995,
                "subscription_id": 120475,
                "event_time": 1516126040
            }"#,
        )
        .unwrap();
        assert_eq!(event.object_id, 9_007_199_254_740_993);
        assert_eq!(event.owner_id, 9_007_199_254_740_995);
    }
}

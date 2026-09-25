use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The primary photo of an activity
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotosSummaryPrimary {
    /// The identifier of the photo
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// Where the photo came from
    #[serde(rename = "source")]
    pub source: Option<i32>,

    /// The unique identifier of the photo
    #[serde(rename = "unique_id")]
    pub unique_id: Option<String>,

    /// URLs of the photo, keyed by size in pixels
    #[serde(rename = "urls")]
    pub urls: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<PhotosSummaryPrimary>(
            r#"{"id": 12345678905, "source": 1, "unique_id": "a1b2c3", "urls": {"100": "https://example.com/100.jpg", "600": "https://example.com/600.jpg"}}"#,
        );
    }
}

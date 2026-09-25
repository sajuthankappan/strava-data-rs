use super::super::models;
use serde::{Deserialize, Serialize};

/// A summary of an activity's photos
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotosSummary {
    /// The number of photos
    #[serde(rename = "count")]
    pub count: Option<i32>,

    /// The activity's primary photo
    #[serde(rename = "primary")]
    pub primary: Option<models::PhotosSummaryPrimary>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<PhotosSummary>(
            r#"{"count": 2, "primary": {"id": 12345678905, "source": 1, "unique_id": "a1b2c3", "urls": {"100": "https://example.com/100.jpg", "600": "https://example.com/600.jpg"}}}"#,
        );
    }
}

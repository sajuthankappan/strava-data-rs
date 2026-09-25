use serde::{Deserialize, Serialize};

/// A summary of an athlete's effort on a segment
#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySegmentEffort {
    /// The unique identifier of this effort
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// The unique identifier of the activity related to this effort
    #[serde(rename = "activity_id")]
    pub activity_id: Option<i64>,

    /// The effort's elapsed time
    #[serde(rename = "elapsed_time")]
    pub elapsed_time: Option<i32>,

    /// The time at which the effort was started.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,

    /// The time at which the effort was started in the local timezone.
    #[serde(rename = "start_date_local")]
    pub start_date_local: Option<String>,

    /// The effort's distance in meters
    #[serde(rename = "distance")]
    pub distance: Option<f32>,

    /// Whether this effort is the current best on the leaderboard
    #[serde(rename = "is_kom")]
    pub is_kom: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<SummarySegmentEffort>(
            r#"{"id": 12345678901, "activity_id": 12345678902, "elapsed_time": 300, "start_date": "2026-09-20T06:10:00Z", "start_date_local": "2026-09-20T11:40:00Z", "distance": 1500.5, "is_kom": false}"#,
        );
    }
}

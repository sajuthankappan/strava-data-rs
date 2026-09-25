use serde::{Deserialize, Serialize};

/// The athlete's personal record effort on a segment
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryPrSegmentEffort {
    /// The unique identifier of the activity related to the PR effort.
    #[serde(rename = "pr_activity_id")]
    pub pr_activity_id: Option<i64>,

    /// The elapsed time ot the PR effort.
    #[serde(rename = "pr_elapsed_time")]
    pub pr_elapsed_time: Option<i32>,

    /// The time at which the PR effort was started.
    #[serde(rename = "pr_date")]
    pub pr_date: Option<String>,

    /// Number of efforts by the authenticated athlete on this segment.
    #[serde(rename = "effort_count")]
    pub effort_count: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<SummaryPrSegmentEffort>(
            r#"{"pr_activity_id": 12345678903, "pr_elapsed_time": 280, "pr_date": "2026-08-01T06:00:00Z", "effort_count": 12}"#,
        );
    }
}

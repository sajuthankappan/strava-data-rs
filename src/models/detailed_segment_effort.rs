use super::super::models;
use serde::{Deserialize, Serialize};

/// An athlete's effort on a segment, as returned in an activity's segment and best efforts
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedSegmentEffort {
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

    /// The name of the segment on which this effort was performed
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The activity this effort belongs to
    #[serde(rename = "activity")]
    pub activity: Option<models::MetaActivity>,

    /// The athlete who made this effort
    #[serde(rename = "athlete")]
    pub athlete: Option<models::MetaAthlete>,

    /// The effort's moving time
    #[serde(rename = "moving_time")]
    pub moving_time: Option<i32>,

    /// The start index of this effort in its activity's stream
    #[serde(rename = "start_index")]
    pub start_index: Option<i32>,

    /// The end index of this effort in its activity's stream
    #[serde(rename = "end_index")]
    pub end_index: Option<i32>,

    /// The effort's average cadence
    #[serde(rename = "average_cadence")]
    pub average_cadence: Option<f32>,

    /// The average wattage of this effort
    #[serde(rename = "average_watts")]
    pub average_watts: Option<f32>,

    /// For riding efforts, whether the wattage was reported by a dedicated recording device
    #[serde(rename = "device_watts")]
    pub device_watts: Option<bool>,

    /// The heart heart rate of the athlete during this effort
    #[serde(rename = "average_heartrate")]
    pub average_heartrate: Option<f32>,

    /// The maximum heart rate of the athlete during this effort
    #[serde(rename = "max_heartrate")]
    pub max_heartrate: Option<f32>,

    /// The segment this effort was made on
    #[serde(rename = "segment")]
    pub segment: Option<models::SummarySegment>,

    /// The rank of the effort on the global leaderboard if it belongs in the top 10 at the time of upload
    #[serde(rename = "kom_rank")]
    pub kom_rank: Option<i32>,

    /// The rank of the effort on the athlete's leaderboard if it belongs in the top 3 at the time of upload
    #[serde(rename = "pr_rank")]
    pub pr_rank: Option<i32>,

    /// Whether this effort should be hidden when viewed within an activity
    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<DetailedSegmentEffort>(
            r#"{"id": 12345678901, "activity_id": 12345678902, "elapsed_time": 300, "start_date": "2026-09-20T06:10:00Z", "start_date_local": "2026-09-20T11:40:00Z", "distance": 1500.5, "is_kom": false, "name": "Hawk Hill", "activity": {"id": 12345678902}, "athlete": {"id": 134815}, "moving_time": 295, "start_index": 10, "end_index": 400, "average_cadence": 85.5, "average_watts": 250.5, "device_watts": true, "average_heartrate": 150.5, "max_heartrate": 172.0, "segment": {"id": 229781, "name": "Hawk Hill", "activity_type": "Ride", "distance": 2684.75, "average_grade": 5.5, "maximum_grade": 14.25, "elevation_high": 245.25, "elevation_low": 92.5, "start_latlng": [37.8331, -122.4834], "end_latlng": [37.8280, -122.4981], "climb_category": 1, "city": "San Francisco", "state": "CA", "country": "United States", "private": false, "athlete_pr_effort": {"pr_activity_id": 12345678903, "pr_elapsed_time": 280, "pr_date": "2026-08-01T06:00:00Z", "effort_count": 12}, "athlete_segment_stats": {"id": 12345678901, "activity_id": 12345678902, "elapsed_time": 300, "start_date": "2026-09-20T06:10:00Z", "start_date_local": "2026-09-20T11:40:00Z", "distance": 1500.5, "is_kom": false}}, "kom_rank": 3, "pr_rank": 1, "hidden": false}"#,
        );
    }
}

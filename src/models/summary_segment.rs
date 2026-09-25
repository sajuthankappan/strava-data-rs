use super::super::models;
use serde::{Deserialize, Serialize};

/// A summary of a segment
#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySegment {
    /// The unique identifier of this segment
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// The name of this segment
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The segment's activity type, `Ride` or `Run`. Kept as a string so unexpected values don't fail to decode
    #[serde(rename = "activity_type")]
    pub activity_type: Option<String>,

    /// The segment's distance, in meters
    #[serde(rename = "distance")]
    pub distance: Option<f32>,

    /// The segment's average grade, in percents
    #[serde(rename = "average_grade")]
    pub average_grade: Option<f32>,

    /// The segments's maximum grade, in percents
    #[serde(rename = "maximum_grade")]
    pub maximum_grade: Option<f32>,

    /// The segments's highest elevation, in meters
    #[serde(rename = "elevation_high")]
    pub elevation_high: Option<f32>,

    /// The segments's lowest elevation, in meters
    #[serde(rename = "elevation_low")]
    pub elevation_low: Option<f32>,

    /// The start coordinates of the segment
    #[serde(rename = "start_latlng")]
    pub start_latlng: Option<models::LatLng>,

    /// The end coordinates of the segment
    #[serde(rename = "end_latlng")]
    pub end_latlng: Option<models::LatLng>,

    /// The category of the climb [0, 5]. Higher is harder ie. 5 is Hors catégorie, 0 is uncategorized in climb_category.
    #[serde(rename = "climb_category")]
    pub climb_category: Option<i32>,

    /// The segments's city.
    #[serde(rename = "city")]
    pub city: Option<String>,

    /// The segments's state or geographical region.
    #[serde(rename = "state")]
    pub state: Option<String>,

    /// The segment's country.
    #[serde(rename = "country")]
    pub country: Option<String>,

    /// Whether this segment is private.
    #[serde(rename = "private")]
    pub private: Option<bool>,

    /// The athlete's personal record effort on this segment
    #[serde(rename = "athlete_pr_effort")]
    pub athlete_pr_effort: Option<models::SummaryPrSegmentEffort>,

    /// The athlete's effort statistics on this segment
    #[serde(rename = "athlete_segment_stats")]
    pub athlete_segment_stats: Option<models::SummarySegmentEffort>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<SummarySegment>(
            r#"{"id": 229781, "name": "Hawk Hill", "activity_type": "Ride", "distance": 2684.75, "average_grade": 5.5, "maximum_grade": 14.25, "elevation_high": 245.25, "elevation_low": 92.5, "start_latlng": [37.8331, -122.4834], "end_latlng": [37.8280, -122.4981], "climb_category": 1, "city": "San Francisco", "state": "CA", "country": "United States", "private": false, "athlete_pr_effort": {"pr_activity_id": 12345678903, "pr_elapsed_time": 280, "pr_date": "2026-08-01T06:00:00Z", "effort_count": 12}, "athlete_segment_stats": {"id": 12345678901, "activity_id": 12345678902, "elapsed_time": 300, "start_date": "2026-09-20T06:10:00Z", "start_date_local": "2026-09-20T11:40:00Z", "distance": 1500.5, "is_kom": false}}"#,
        );
    }
}

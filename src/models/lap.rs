use super::super::models;
use serde::{Deserialize, Serialize};

/// A lap of an activity
#[derive(Debug, Serialize, Deserialize)]
pub struct Lap {
    /// The unique identifier of this lap
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// The activity this lap belongs to
    #[serde(rename = "activity")]
    pub activity: Option<models::MetaActivity>,

    /// The athlete who recorded this lap
    #[serde(rename = "athlete")]
    pub athlete: Option<models::MetaAthlete>,

    /// The lap's average cadence
    #[serde(rename = "average_cadence")]
    pub average_cadence: Option<f32>,

    /// The lap's average speed
    #[serde(rename = "average_speed")]
    pub average_speed: Option<f32>,

    /// The lap's distance, in meters
    #[serde(rename = "distance")]
    pub distance: Option<f32>,

    /// The lap's elapsed time, in seconds
    #[serde(rename = "elapsed_time")]
    pub elapsed_time: Option<i32>,

    /// The start index of this effort in its activity's stream
    #[serde(rename = "start_index")]
    pub start_index: Option<i32>,

    /// The end index of this effort in its activity's stream
    #[serde(rename = "end_index")]
    pub end_index: Option<i32>,

    /// The index of this lap in the activity it belongs to
    #[serde(rename = "lap_index")]
    pub lap_index: Option<i32>,

    /// The maximum speed of this lat, in meters per second
    #[serde(rename = "max_speed")]
    pub max_speed: Option<f32>,

    /// The lap's moving time, in seconds
    #[serde(rename = "moving_time")]
    pub moving_time: Option<i32>,

    /// The name of the lap
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The athlete's pace zone during this lap
    #[serde(rename = "pace_zone")]
    pub pace_zone: Option<i32>,

    /// The number of this lap, starting from 1
    #[serde(rename = "split")]
    pub split: Option<i32>,

    /// The time at which the lap was started.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,

    /// The time at which the lap was started in the local timezone.
    #[serde(rename = "start_date_local")]
    pub start_date_local: Option<String>,

    /// The elevation gain of this lap, in meters
    #[serde(rename = "total_elevation_gain")]
    pub total_elevation_gain: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<Lap>(
            r#"{"id": 12345678904, "activity": {"id": 12345678902}, "athlete": {"id": 134815}, "average_cadence": 88.5, "average_speed": 3.5, "distance": 5000.25, "elapsed_time": 1500, "start_index": 0, "end_index": 1499, "lap_index": 1, "max_speed": 5.25, "moving_time": 1480, "name": "Lap 1", "pace_zone": 3, "split": 1, "start_date": "2026-09-20T06:00:00Z", "start_date_local": "2026-09-20T11:30:00Z", "total_elevation_gain": 25.5}"#,
        );
    }
}

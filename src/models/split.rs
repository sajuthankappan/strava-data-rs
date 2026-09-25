use serde::{Deserialize, Serialize};

/// A split of an activity, in metric or imperial units
#[derive(Debug, Serialize, Deserialize)]
pub struct Split {
    /// The average speed of this split, in meters per second
    #[serde(rename = "average_speed")]
    pub average_speed: Option<f32>,

    /// The distance of this split, in meters
    #[serde(rename = "distance")]
    pub distance: Option<f32>,

    /// The elapsed time of this split, in seconds
    #[serde(rename = "elapsed_time")]
    pub elapsed_time: Option<i32>,

    /// The elevation difference of this split, in meters
    #[serde(rename = "elevation_difference")]
    pub elevation_difference: Option<f32>,

    /// The pacing zone of this split
    #[serde(rename = "pace_zone")]
    pub pace_zone: Option<i32>,

    /// The moving time of this split, in seconds
    #[serde(rename = "moving_time")]
    pub moving_time: Option<i32>,

    /// The number of this split, starting from 1
    #[serde(rename = "split")]
    pub split: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<Split>(
            r#"{"average_speed": 3.25, "distance": 1000.5, "elapsed_time": 310, "elevation_difference": 4.5, "pace_zone": 2, "moving_time": 305, "split": 1}"#,
        );
    }
}

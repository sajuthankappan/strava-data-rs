use super::super::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryActivity {
    /// The unique identifier of the activity
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The identifier provided at upload time
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,

    /// The identifier of the upload that resulted in this activity
    #[serde(rename = "upload_id")]
    pub upload_id: Option<i64>,

    #[serde(rename = "athlete")]
    pub athlete: Option<models::MetaAthlete>,

    /// The name of the activity
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The activity's distance, in meters
    #[serde(rename = "distance")]
    pub distance: Option<f32>,

    /// The activity's moving time, in seconds
    #[serde(rename = "moving_time")]
    pub moving_time: Option<i32>,

    /// The activity's elapsed time, in seconds
    #[serde(rename = "elapsed_time")]
    pub elapsed_time: Option<i32>,

    /// The activity's total elevation gain.
    #[serde(rename = "total_elevation_gain")]
    pub total_elevation_gain: Option<f32>,

    /// The activity's highest elevation, in meters
    #[serde(rename = "elev_high")]
    pub elev_high: Option<f32>,

    /// The activity's lowest elevation, in meters
    #[serde(rename = "elev_low")]
    pub elev_low: Option<f32>,

    #[serde(rename = "type")]
    pub activity_type: Option<models::ActivityType>,

    /// The activity's sport type
    #[serde(rename = "sport_type")]
    pub sport_type: Option<models::SportType>,

    /// The time at which the activity was started.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,

    /// The time at which the activity was started in the local timezone.
    #[serde(rename = "start_date_local")]
    pub start_date_local: Option<String>,

    /// The timezone of the activity
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,

    /// The UTC offset of the local timezone
    #[serde(rename = "utc_offset")]
    pub utc_offset: Option<f32>,

    /// The start coordinates of the activity
    #[serde(rename = "start_latlng")]
    pub start_latlng: Option<models::LatLng>,

    /// The end coordinates of the activity
    #[serde(rename = "end_latlng")]
    pub end_latlng: Option<models::LatLng>,

    /// The number of achievements gained during this activity
    #[serde(rename = "achievement_count")]
    pub achievement_count: Option<i32>,

    /// The number of kudos given for this activity
    #[serde(rename = "kudos_count")]
    pub kudos_count: Option<i32>,

    /// The number of comments for this activity
    #[serde(rename = "comment_count")]
    pub comment_count: Option<i32>,

    /// The number of athletes for taking part in a group activity
    #[serde(rename = "athlete_count")]
    pub athlete_count: Option<i32>,

    /// The number of Instagram photos for this activity
    #[serde(rename = "photo_count")]
    pub photo_count: Option<i32>,

    /// The number of Instagram and Strava photos for this activity
    #[serde(rename = "total_photo_count")]
    pub total_photo_count: Option<i32>,

    #[serde(rename = "map")]
    pub map: Option<models::PolylineMap>,

    /// The name of the device used to record the activity
    #[serde(rename = "device_name")]
    pub device_name: Option<String>,

    /// Whether this activity was recorded on a training machine
    #[serde(rename = "trainer")]
    pub trainer: Option<bool>,

    /// Whether this activity is a commute
    #[serde(rename = "commute")]
    pub commute: Option<bool>,

    /// Whether this activity was created manually
    #[serde(rename = "manual")]
    pub manual: Option<bool>,

    /// Whether this activity is private
    #[serde(rename = "private")]
    pub private: Option<bool>,

    /// Whether this activity is flagged
    #[serde(rename = "flagged")]
    pub flagged: Option<bool>,

    /// The activity's workout type
    #[serde(rename = "workout_type")]
    pub workout_type: Option<i32>,

    /// The unique identifier of the upload in string format
    #[serde(rename = "upload_id_str")]
    pub upload_id_str: Option<String>,

    /// The activity's average speed, in meters per second
    #[serde(rename = "average_speed")]
    pub average_speed: Option<f32>,

    /// The activity's max speed, in meters per second
    #[serde(rename = "max_speed")]
    pub max_speed: Option<f32>,

    /// Whether the logged-in athlete has kudoed this activity
    #[serde(rename = "has_kudoed")]
    pub has_kudoed: Option<bool>,

    /// Whether the activity is muted
    #[serde(rename = "hide_from_home")]
    pub hide_from_home: Option<bool>,

    /// The id of the gear for the activity
    #[serde(rename = "gear_id")]
    pub gear_id: Option<String>,

    /// The total work done in kilojoules during this activity. Rides only
    #[serde(rename = "kilojoules")]
    pub kilojoules: Option<f32>,

    /// Average power output in watts during this activity. Rides only
    #[serde(rename = "average_watts")]
    pub average_watts: Option<f32>,

    /// Whether the watts are from a power meter, false if estimated
    #[serde(rename = "device_watts")]
    pub device_watts: Option<bool>,

    /// Rides with power meter data only
    #[serde(rename = "max_watts")]
    pub max_watts: Option<i32>,

    /// Similar to Normalized Power. Rides with power meter data only
    #[serde(rename = "weighted_average_watts")]
    pub weighted_average_watts: Option<i32>,
}

impl From<models::DetailedActivity> for SummaryActivity {
    /// Keep the fields a summary activity has, dropping the detail-only ones
    fn from(activity: models::DetailedActivity) -> Self {
        // Listing every field, without `..`, makes adding a field to `DetailedActivity`
        // a compile error here until it is either carried over or dropped
        let models::DetailedActivity {
            id,
            external_id,
            upload_id,
            athlete,
            name,
            distance,
            moving_time,
            elapsed_time,
            total_elevation_gain,
            elev_high,
            elev_low,
            activity_type,
            sport_type,
            start_date,
            start_date_local,
            timezone,
            utc_offset,
            start_latlng,
            end_latlng,
            achievement_count,
            kudos_count,
            comment_count,
            athlete_count,
            photo_count,
            total_photo_count,
            map,
            trainer,
            commute,
            manual,
            private,
            flagged,
            workout_type,
            upload_id_str,
            average_speed,
            max_speed,
            has_kudoed,
            hide_from_home,
            gear_id,
            kilojoules,
            average_watts,
            device_watts,
            max_watts,
            weighted_average_watts,
            device_name,
            description: _,
            calories: _,
            embed_token: _,
            photos: _,
            gear: _,
            segment_efforts: _,
            splits_metric: _,
            splits_standard: _,
            laps: _,
            best_efforts: _,
        } = activity;
        SummaryActivity {
            id,
            external_id,
            upload_id,
            athlete,
            name,
            distance,
            moving_time,
            elapsed_time,
            total_elevation_gain,
            elev_high,
            elev_low,
            activity_type,
            sport_type,
            start_date,
            start_date_local,
            timezone,
            utc_offset,
            start_latlng,
            end_latlng,
            achievement_count,
            kudos_count,
            comment_count,
            athlete_count,
            photo_count,
            total_photo_count,
            map,
            trainer,
            commute,
            manual,
            private,
            flagged,
            workout_type,
            upload_id_str,
            average_speed,
            max_speed,
            has_kudoed,
            hide_from_home,
            gear_id,
            kilojoules,
            average_watts,
            device_watts,
            max_watts,
            weighted_average_watts,
            device_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_list_item() {
        let activity: SummaryActivity = serde_json::from_str(
            r#"{
                "id": 1,
                "type": "Ride",
                "sport_type": "GravelRide",
                "hide_from_home": true,
                "device_name": "Garmin Forerunner 965",
                "weighted_average_watts": 210,
                "start_latlng": [51.5074, -0.1278],
                "map": {"id": "a1", "summary_polyline": "abc", "polyline": null}
            }"#,
        )
        .unwrap();
        assert_eq!(activity.id, Some(1));
        assert_eq!(activity.sport_type, Some(models::SportType::GravelRide));
        assert_eq!(activity.hide_from_home, Some(true));
        assert_eq!(
            activity.device_name.as_deref(),
            Some("Garmin Forerunner 965")
        );
        assert_eq!(activity.weighted_average_watts, Some(210));
        assert_eq!(activity.start_latlng.unwrap().latitude(), Some(51.5074));
        let map = activity.map.unwrap();
        assert_eq!(map.summary_polyline.as_deref(), Some("abc"));
        assert!(map.polyline.is_none());
    }

    #[test]
    fn converts_from_detailed_activity() {
        // Every field has a distinct value, so a field that is dropped or mixed up shows
        // as a difference from the summary deserialized directly
        let json = r#"{
            "id": 1,
            "external_id": "garmin_push_1",
            "upload_id": 2,
            "athlete": {"id": 3},
            "name": "Morning Ride",
            "distance": 40000.5,
            "moving_time": 5400,
            "elapsed_time": 6000,
            "total_elevation_gain": 350.5,
            "elev_high": 120.5,
            "elev_low": 10.5,
            "type": "Ride",
            "sport_type": "GravelRide",
            "start_date": "2026-09-20T06:00:00Z",
            "start_date_local": "2026-09-20T11:30:00Z",
            "timezone": "(GMT+05:30) Asia/Kolkata",
            "utc_offset": 19800.0,
            "start_latlng": [51.5074, -0.1278],
            "end_latlng": [48.8566, 2.3522],
            "achievement_count": 4,
            "kudos_count": 5,
            "comment_count": 6,
            "athlete_count": 7,
            "photo_count": 8,
            "total_photo_count": 9,
            "map": {"id": "a1", "polyline": "full", "summary_polyline": "abc"},
            "trainer": false,
            "commute": true,
            "manual": false,
            "private": true,
            "flagged": false,
            "workout_type": 10,
            "upload_id_str": "2",
            "average_speed": 7.5,
            "max_speed": 15.5,
            "has_kudoed": true,
            "hide_from_home": false,
            "gear_id": "b123",
            "kilojoules": 900.5,
            "average_watts": 180.5,
            "device_watts": true,
            "max_watts": 11,
            "weighted_average_watts": 12,
            "device_name": "Garmin Edge 840",
            "description": "Easy spin",
            "calories": 950.5,
            "embed_token": "token"
        }"#;
        let detailed: models::DetailedActivity = serde_json::from_str(json).unwrap();
        let direct: SummaryActivity = serde_json::from_str(json).unwrap();

        let converted = SummaryActivity::from(detailed);
        let converted = serde_json::to_value(converted).unwrap();
        assert_eq!(converted, serde_json::to_value(direct).unwrap());
        // The fixture covers every summary field
        let nulls: Vec<_> = converted
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, value)| value.is_null())
            .map(|(key, _)| key)
            .collect();
        assert!(
            nulls.is_empty(),
            "fields missing from the fixture: {nulls:?}"
        );
    }
}

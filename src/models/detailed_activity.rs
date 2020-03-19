use serde::{Deserialize, Serialize};
use super::super::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedActivity {
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
  #[serde(rename = "<UtcOffset>")]
  pub utc_offset: Option<i32>,
  
  //#[serde(rename = "start_latlng")]
  //start_latlng: Option<models::LatLng>,
  //#[serde(rename = "end_latlng")]
  //end_latlng: Option<models::LatLng>,
  
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
  weighted_average_watts: Option<i32>,
  
  /// The description of the activity
  #[serde(rename = "description")]
  pub description: Option<String>,
  
  /*#[serde(rename = "photos")] //TODO
  photos: Option<models::PhotosSummary>,
  #[serde(rename = "gear")]
  gear: Option<models::SummaryGear>,*/
  
  /// The number of kilocalories consumed during this activity
  #[serde(rename = "calories")]
  pub calories: Option<f32>,
  
  //#[serde(rename = "segment_efforts")]  //TODO: Do we need this?
  //segment_efforts: Option<Vec<::models::DetailedSegmentEffort>>,
  
  /// The name of the device used to record the activity
  #[serde(rename = "device_name")]
  pub device_name: Option<String>,
  
  /// The token used to embed a Strava activity
  #[serde(rename = "embed_token")]
  pub embed_token: Option<String>,
  
  //TODO: Complete these
  /*/// The splits of this activity in metric units (for runs)
  #[serde(rename = "splits_metric")]
  splits_metric: Option<Vec<::models::Split>>,
  /// The splits of this activity in imperial units (for runs)
  #[serde(rename = "splits_standard")]
  splits_standard: Option<Vec<::models::Split>>,
  #[serde(rename = "laps")]
  laps: Option<Vec<::models::Lap>>,
  #[serde(rename = "best_efforts")]
  best_efforts: Option<Vec<::models::DetailedSegmentEffort>>*/
}
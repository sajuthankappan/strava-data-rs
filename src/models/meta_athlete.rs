use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaAthlete {
  /// The unique identifier of the athlete
  #[serde(rename = "id")]
  pub id: Option<i32>
}

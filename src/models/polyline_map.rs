use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PolylineMap {
  /// The identifier of the map
  #[serde(rename = "id")]
  id: Option<String>,
  /// The polyline of the map, only returned on detailed representation of an object
  #[serde(rename = "polyline")]
  polyline: Option<String>,
  /// The summary polyline of the map
  #[serde(rename = "summary_polyline")]
  summary_polyline: Option<String>
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PolylineMap {
  /// The identifier of the map
  pub id: Option<String>,
  /// The polyline of the map, only returned on detailed representation of an object
  pub polyline: Option<String>,
  /// The summary polyline of the map
  pub summary_polyline: Option<String>
}

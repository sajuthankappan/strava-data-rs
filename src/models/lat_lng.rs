use serde::{Deserialize, Serialize};

/// A pair of latitude/longitude coordinates, sent by Strava as a `[lat, lng]` array.
/// Activities without GPS data (manual entries, treadmill or trainer sessions) send an empty array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LatLng(Vec<f64>);

impl LatLng {
    /// The latitude, if coordinates are present
    pub fn latitude(&self) -> Option<f64> {
        self.coordinates().map(|(lat, _)| lat)
    }

    /// The longitude, if coordinates are present
    pub fn longitude(&self) -> Option<f64> {
        self.coordinates().map(|(_, lng)| lng)
    }

    fn coordinates(&self) -> Option<(f64, f64)> {
        match self.0.as_slice() {
            [lat, lng] => Some((*lat, *lng)),
            _ => None,
        }
    }
}

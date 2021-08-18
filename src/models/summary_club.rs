use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummaryClub {
  /// The club's unique identifier.
  pub id: Option<i32>,
  /// Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\"
  pub resource_state: Option<i32>,
  /// The club's name.
  pub name: Option<String>,
  /// URL to a 60x60 pixel profile picture.
  pub profile_medium: Option<String>,
  /// URL to a ~1185x580 pixel cover photo.
  pub cover_photo: Option<String>,
  /// URL to a ~360x176  pixel cover photo.
  pub cover_photo_small: Option<String>,
  pub sport_type: Option<String>,
  /// The club's city.
  pub city: Option<String>,
  /// The club's state or geographical region.
  pub state: Option<String>,
  /// The club's country.
  pub country: Option<String>,
  /// Whether the club is private.
  pub private: Option<bool>,
  /// The club's member count.
  pub member_count: Option<i32>,
  /// Whether the club is featured or not.
  pub featured: Option<bool>,
  /// Whether the club is verified or not.
  pub verified: Option<bool>,
  /// The club's vanity URL.
  pub url: Option<String>,
}

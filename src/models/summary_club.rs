use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SummaryClub {
  /// The club's unique identifier.
  id: Option<i32>,
  /// Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\"
  resource_state: Option<i32>,
  /// The club's name.
  name: Option<String>,
  /// URL to a 60x60 pixel profile picture.
  profile_medium: Option<String>,
  /// URL to a ~1185x580 pixel cover photo.
  cover_photo: Option<String>,
  /// URL to a ~360x176  pixel cover photo.
  cover_photo_small: Option<String>,
  sport_type: Option<String>,
  /// The club's city.
  city: Option<String>,
  /// The club's state or geographical region.
  state: Option<String>,
  /// The club's country.
  country: Option<String>,
  /// Whether the club is private.
  private: Option<bool>,
  /// The club's member count.
  member_count: Option<i32>,
  /// Whether the club is featured or not.
  featured: Option<bool>,
  /// Whether the club is verified or not.
  verified: Option<bool>,
  /// The club's vanity URL.
  url: Option<String>,
}

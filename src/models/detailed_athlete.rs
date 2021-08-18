use super::{SummaryClub, SummaryGear};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DetailedAthlete {
    /// The unique identifier of the athlete
    pub id: Option<i32>,
    /// Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\"
    pub resource_state: Option<i32>,
    /// The athlete's first name.
    pub firstname: Option<String>,
    /// The athlete's last name.
    pub lastname: Option<String>,
    /// URL to a 62x62 pixel profile picture.
    pub profile_medium: Option<String>,
    /// URL to a 124x124 pixel profile picture.
    pub profile: Option<String>,
    /// The athlete's city.
    pub city: Option<String>,
    /// The athlete's state or geographical region.
    pub state: Option<String>,
    /// The athlete's country.
    pub country: Option<String>,
    /// The athlete's sex.
    pub sex: Option<String>,
    /// Whether the athlete has any Summit subscription.
    pub summit: Option<bool>,
    /// The time at which the athlete was created.
    pub created_at: Option<String>,
    /// The time at which the athlete was last updated.
    pub updated_at: Option<String>,
    /// The athlete's follower count.
    pub follower_count: Option<i32>,
    /// The athlete's friend count.
    pub friend_count: Option<i32>,
    /// The athlete's preferred unit system.
    pub measurement_preference: Option<String>,
    /// The athlete's FTP (Functional Threshold Power).
    pub ftp: Option<i32>,
    /// The athlete's weight.
    pub weight: Option<f32>,
    /// The athlete's clubs.
    pub clubs: Option<Vec<SummaryClub>>,
    /// The athlete's bikes.
    pub bikes: Option<Vec<SummaryGear>>,
    /// The athlete's shoes.
    pub shoes: Option<Vec<SummaryGear>>,
}

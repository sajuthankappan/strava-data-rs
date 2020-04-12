use super::{SummaryClub, SummaryGear};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct DetailedAthlete {
    /// The unique identifier of the athlete
    id: Option<i32>,
    /// Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\"
    resource_state: Option<i32>,
    /// The athlete's first name.
    firstname: Option<String>,
    /// The athlete's last name.
    lastname: Option<String>,
    /// URL to a 62x62 pixel profile picture.
    profile_medium: Option<String>,
    /// URL to a 124x124 pixel profile picture.
    profile: Option<String>,
    /// The athlete's city.
    city: Option<String>,
    /// The athlete's state or geographical region.
    state: Option<String>,
    /// The athlete's country.
    country: Option<String>,
    /// The athlete's sex.
    sex: Option<String>,
    /// Whether the athlete has any Summit subscription.
    summit: Option<bool>,
    /// The time at which the athlete was created.
    created_at: Option<String>,
    /// The time at which the athlete was last updated.
    updated_at: Option<String>,
    /// The athlete's follower count.
    follower_count: Option<i32>,
    /// The athlete's friend count.
    friend_count: Option<i32>,
    /// The athlete's preferred unit system.
    measurement_preference: Option<String>,
    /// The athlete's FTP (Functional Threshold Power).
    ftp: Option<i32>,
    /// The athlete's weight.
    weight: Option<f32>,
    /// The athlete's clubs.
    clubs: Option<Vec<SummaryClub>>,
    /// The athlete's bikes.
    bikes: Option<Vec<SummaryGear>>,
    /// The athlete's shoes.
    shoes: Option<Vec<SummaryGear>>,
}

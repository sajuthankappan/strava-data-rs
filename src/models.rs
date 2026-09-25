mod activity_type;
pub use activity_type::{ActivityType, ParseActivityTypeError};

mod sport_type;
pub use sport_type::SportType;

mod detailed_activity;
pub use detailed_activity::DetailedActivity;

mod summary_activity;
pub use summary_activity::SummaryActivity;

mod meta_activity;
pub use meta_activity::MetaActivity;

mod photos_summary;
pub use photos_summary::PhotosSummary;

mod photos_summary_primary;
pub use photos_summary_primary::PhotosSummaryPrimary;

mod split;
pub use split::Split;

mod lap;
pub use lap::Lap;

mod detailed_segment_effort;
pub use detailed_segment_effort::DetailedSegmentEffort;

mod summary_segment_effort;
pub use summary_segment_effort::SummarySegmentEffort;

mod summary_pr_segment_effort;
pub use summary_pr_segment_effort::SummaryPrSegmentEffort;

mod summary_segment;
pub use summary_segment::SummarySegment;

mod detailed_athlete;
pub use detailed_athlete::DetailedAthlete;

mod summary_athlete;
pub use summary_athlete::SummaryAthlete;

mod summary_club;
pub use summary_club::SummaryClub;

mod summary_gear;
pub use summary_gear::SummaryGear;

mod lat_lng;
pub use lat_lng::LatLng;

mod meta_athlete;
pub use meta_athlete::MetaAthlete;

mod polyline_map;
pub use polyline_map::PolylineMap;

pub mod webhooks;

/// Assert that `json`, which should set every field, deserializes into `T` and serializes back unchanged
#[cfg(test)]
pub(crate) fn assert_round_trips<T>(json: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let expected: serde_json::Value = serde_json::from_str(json).unwrap();
    let model: T = serde_json::from_str(json).unwrap();
    assert_eq!(serde_json::to_value(model).unwrap(), expected);
}

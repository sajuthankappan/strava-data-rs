use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    Golf,
    Handcycle,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    Ride,
    RockClimbing,
    RollerSki,
    Rowing,
    Run,
    Sail,
    Skateboard,
    Snowboard,
    Snowshoe,
    Soccer,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    Velomobile,
    VirtualRide,
    VirtualRun,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
}

impl ActivityType {
    /// The name of the activity type, identical to its serialized value
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityType::AlpineSki => "AlpineSki",
            ActivityType::BackcountrySki => "BackcountrySki",
            ActivityType::Canoeing => "Canoeing",
            ActivityType::Crossfit => "Crossfit",
            ActivityType::EBikeRide => "EBikeRide",
            ActivityType::Elliptical => "Elliptical",
            ActivityType::Golf => "Golf",
            ActivityType::Handcycle => "Handcycle",
            ActivityType::Hike => "Hike",
            ActivityType::IceSkate => "IceSkate",
            ActivityType::InlineSkate => "InlineSkate",
            ActivityType::Kayaking => "Kayaking",
            ActivityType::Kitesurf => "Kitesurf",
            ActivityType::NordicSki => "NordicSki",
            ActivityType::Ride => "Ride",
            ActivityType::RockClimbing => "RockClimbing",
            ActivityType::RollerSki => "RollerSki",
            ActivityType::Rowing => "Rowing",
            ActivityType::Run => "Run",
            ActivityType::Sail => "Sail",
            ActivityType::Skateboard => "Skateboard",
            ActivityType::Snowboard => "Snowboard",
            ActivityType::Snowshoe => "Snowshoe",
            ActivityType::Soccer => "Soccer",
            ActivityType::StairStepper => "StairStepper",
            ActivityType::StandUpPaddling => "StandUpPaddling",
            ActivityType::Surfing => "Surfing",
            ActivityType::Swim => "Swim",
            ActivityType::Velomobile => "Velomobile",
            ActivityType::VirtualRide => "VirtualRide",
            ActivityType::VirtualRun => "VirtualRun",
            ActivityType::Walk => "Walk",
            ActivityType::WeightTraining => "WeightTraining",
            ActivityType::Wheelchair => "Wheelchair",
            ActivityType::Windsurf => "Windsurf",
            ActivityType::Workout => "Workout",
            ActivityType::Yoga => "Yoga",
        }
    }
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The error returned when parsing an unknown activity type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseActivityTypeError(String);

impl fmt::Display for ParseActivityTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown activity type: {}", self.0)
    }
}

impl std::error::Error for ParseActivityTypeError {}

impl FromStr for ActivityType {
    type Err = ParseActivityTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AlpineSki" => Ok(ActivityType::AlpineSki),
            "BackcountrySki" => Ok(ActivityType::BackcountrySki),
            "Canoeing" => Ok(ActivityType::Canoeing),
            "Crossfit" => Ok(ActivityType::Crossfit),
            "EBikeRide" => Ok(ActivityType::EBikeRide),
            "Elliptical" => Ok(ActivityType::Elliptical),
            "Golf" => Ok(ActivityType::Golf),
            "Handcycle" => Ok(ActivityType::Handcycle),
            "Hike" => Ok(ActivityType::Hike),
            "IceSkate" => Ok(ActivityType::IceSkate),
            "InlineSkate" => Ok(ActivityType::InlineSkate),
            "Kayaking" => Ok(ActivityType::Kayaking),
            "Kitesurf" => Ok(ActivityType::Kitesurf),
            "NordicSki" => Ok(ActivityType::NordicSki),
            "Ride" => Ok(ActivityType::Ride),
            "RockClimbing" => Ok(ActivityType::RockClimbing),
            "RollerSki" => Ok(ActivityType::RollerSki),
            "Rowing" => Ok(ActivityType::Rowing),
            "Run" => Ok(ActivityType::Run),
            "Sail" => Ok(ActivityType::Sail),
            "Skateboard" => Ok(ActivityType::Skateboard),
            "Snowboard" => Ok(ActivityType::Snowboard),
            "Snowshoe" => Ok(ActivityType::Snowshoe),
            "Soccer" => Ok(ActivityType::Soccer),
            "StairStepper" => Ok(ActivityType::StairStepper),
            "StandUpPaddling" => Ok(ActivityType::StandUpPaddling),
            "Surfing" => Ok(ActivityType::Surfing),
            "Swim" => Ok(ActivityType::Swim),
            "Velomobile" => Ok(ActivityType::Velomobile),
            "VirtualRide" => Ok(ActivityType::VirtualRide),
            "VirtualRun" => Ok(ActivityType::VirtualRun),
            "Walk" => Ok(ActivityType::Walk),
            "WeightTraining" => Ok(ActivityType::WeightTraining),
            "Wheelchair" => Ok(ActivityType::Wheelchair),
            "Windsurf" => Ok(ActivityType::Windsurf),
            "Workout" => Ok(ActivityType::Workout),
            "Yoga" => Ok(ActivityType::Yoga),
            _ => Err(ParseActivityTypeError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lists every variant once, generating both the list the tests iterate over and an
    // exhaustive match, so adding a variant without updating this list fails to compile.
    macro_rules! all_variants {
        ($($variant:ident,)*) => {
            #[allow(dead_code)]
            fn exhaustive(activity_type: &ActivityType) {
                match activity_type {
                    $(ActivityType::$variant => {})*
                }
            }

            const ALL: &[ActivityType] = &[$(ActivityType::$variant,)*];
        };
    }

    all_variants! {
        AlpineSki,
        BackcountrySki,
        Canoeing,
        Crossfit,
        EBikeRide,
        Elliptical,
        Golf,
        Handcycle,
        Hike,
        IceSkate,
        InlineSkate,
        Kayaking,
        Kitesurf,
        NordicSki,
        Ride,
        RockClimbing,
        RollerSki,
        Rowing,
        Run,
        Sail,
        Skateboard,
        Snowboard,
        Snowshoe,
        Soccer,
        StairStepper,
        StandUpPaddling,
        Surfing,
        Swim,
        Velomobile,
        VirtualRide,
        VirtualRun,
        Walk,
        WeightTraining,
        Wheelchair,
        Windsurf,
        Workout,
        Yoga,
    }

    #[test]
    fn as_str_matches_serde() {
        for activity_type in ALL {
            let json = serde_json::to_string(activity_type).unwrap();
            assert_eq!(activity_type.as_str(), json.trim_matches('"'));
            assert_eq!(activity_type.to_string(), activity_type.as_str());
        }
    }

    #[test]
    fn from_str_round_trips() {
        for activity_type in ALL {
            let parsed: ActivityType = activity_type.as_str().parse().unwrap();
            assert_eq!(parsed.as_str(), activity_type.as_str());
        }
        assert!("NotAnActivity".parse::<ActivityType>().is_err());
    }
}

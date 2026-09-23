use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

/// The sport type of an activity. Unlike `ActivityType`, this includes newer types such as
/// `MountainBikeRide`. Strava keeps adding sport types, so values this crate does not know yet
/// are kept as `Other`, with the exact string Strava sent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SportType {
    AlpineSki,
    BackcountrySki,
    Badminton,
    Basketball,
    Canoeing,
    Cricket,
    Crossfit,
    Dance,
    EBikeRide,
    Elliptical,
    EMountainBikeRide,
    Golf,
    GravelRide,
    Handcycle,
    HighIntensityIntervalTraining,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    MountainBikeRide,
    NordicSki,
    Padel,
    PhysicalTherapy,
    Pickleball,
    Pilates,
    Racquetball,
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
    Squash,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    TableTennis,
    Tennis,
    TrailRun,
    Velomobile,
    VirtualRide,
    VirtualRow,
    VirtualRun,
    Volleyball,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
    /// A sport type not known to this version of the crate
    Other(String),
}

impl SportType {
    /// The name of the sport type, identical to its serialized value
    pub fn as_str(&self) -> &str {
        match self {
            SportType::AlpineSki => "AlpineSki",
            SportType::BackcountrySki => "BackcountrySki",
            SportType::Badminton => "Badminton",
            SportType::Basketball => "Basketball",
            SportType::Canoeing => "Canoeing",
            SportType::Cricket => "Cricket",
            SportType::Crossfit => "Crossfit",
            SportType::Dance => "Dance",
            SportType::EBikeRide => "EBikeRide",
            SportType::Elliptical => "Elliptical",
            SportType::EMountainBikeRide => "EMountainBikeRide",
            SportType::Golf => "Golf",
            SportType::GravelRide => "GravelRide",
            SportType::Handcycle => "Handcycle",
            SportType::HighIntensityIntervalTraining => "HighIntensityIntervalTraining",
            SportType::Hike => "Hike",
            SportType::IceSkate => "IceSkate",
            SportType::InlineSkate => "InlineSkate",
            SportType::Kayaking => "Kayaking",
            SportType::Kitesurf => "Kitesurf",
            SportType::MountainBikeRide => "MountainBikeRide",
            SportType::NordicSki => "NordicSki",
            SportType::Padel => "Padel",
            SportType::PhysicalTherapy => "PhysicalTherapy",
            SportType::Pickleball => "Pickleball",
            SportType::Pilates => "Pilates",
            SportType::Racquetball => "Racquetball",
            SportType::Ride => "Ride",
            SportType::RockClimbing => "RockClimbing",
            SportType::RollerSki => "RollerSki",
            SportType::Rowing => "Rowing",
            SportType::Run => "Run",
            SportType::Sail => "Sail",
            SportType::Skateboard => "Skateboard",
            SportType::Snowboard => "Snowboard",
            SportType::Snowshoe => "Snowshoe",
            SportType::Soccer => "Soccer",
            SportType::Squash => "Squash",
            SportType::StairStepper => "StairStepper",
            SportType::StandUpPaddling => "StandUpPaddling",
            SportType::Surfing => "Surfing",
            SportType::Swim => "Swim",
            SportType::TableTennis => "TableTennis",
            SportType::Tennis => "Tennis",
            SportType::TrailRun => "TrailRun",
            SportType::Velomobile => "Velomobile",
            SportType::VirtualRide => "VirtualRide",
            SportType::VirtualRow => "VirtualRow",
            SportType::VirtualRun => "VirtualRun",
            SportType::Volleyball => "Volleyball",
            SportType::Walk => "Walk",
            SportType::WeightTraining => "WeightTraining",
            SportType::Wheelchair => "Wheelchair",
            SportType::Windsurf => "Windsurf",
            SportType::Workout => "Workout",
            SportType::Yoga => "Yoga",
            SportType::Other(value) => value,
        }
    }
}

impl fmt::Display for SportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SportType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AlpineSki" => SportType::AlpineSki,
            "BackcountrySki" => SportType::BackcountrySki,
            "Badminton" => SportType::Badminton,
            "Basketball" => SportType::Basketball,
            "Canoeing" => SportType::Canoeing,
            "Cricket" => SportType::Cricket,
            "Crossfit" => SportType::Crossfit,
            "Dance" => SportType::Dance,
            "EBikeRide" => SportType::EBikeRide,
            "Elliptical" => SportType::Elliptical,
            "EMountainBikeRide" => SportType::EMountainBikeRide,
            "Golf" => SportType::Golf,
            "GravelRide" => SportType::GravelRide,
            "Handcycle" => SportType::Handcycle,
            "HighIntensityIntervalTraining" => SportType::HighIntensityIntervalTraining,
            "Hike" => SportType::Hike,
            "IceSkate" => SportType::IceSkate,
            "InlineSkate" => SportType::InlineSkate,
            "Kayaking" => SportType::Kayaking,
            "Kitesurf" => SportType::Kitesurf,
            "MountainBikeRide" => SportType::MountainBikeRide,
            "NordicSki" => SportType::NordicSki,
            "Padel" => SportType::Padel,
            "PhysicalTherapy" => SportType::PhysicalTherapy,
            "Pickleball" => SportType::Pickleball,
            "Pilates" => SportType::Pilates,
            "Racquetball" => SportType::Racquetball,
            "Ride" => SportType::Ride,
            "RockClimbing" => SportType::RockClimbing,
            "RollerSki" => SportType::RollerSki,
            "Rowing" => SportType::Rowing,
            "Run" => SportType::Run,
            "Sail" => SportType::Sail,
            "Skateboard" => SportType::Skateboard,
            "Snowboard" => SportType::Snowboard,
            "Snowshoe" => SportType::Snowshoe,
            "Soccer" => SportType::Soccer,
            "Squash" => SportType::Squash,
            "StairStepper" => SportType::StairStepper,
            "StandUpPaddling" => SportType::StandUpPaddling,
            "Surfing" => SportType::Surfing,
            "Swim" => SportType::Swim,
            "TableTennis" => SportType::TableTennis,
            "Tennis" => SportType::Tennis,
            "TrailRun" => SportType::TrailRun,
            "Velomobile" => SportType::Velomobile,
            "VirtualRide" => SportType::VirtualRide,
            "VirtualRow" => SportType::VirtualRow,
            "VirtualRun" => SportType::VirtualRun,
            "Volleyball" => SportType::Volleyball,
            "Walk" => SportType::Walk,
            "WeightTraining" => SportType::WeightTraining,
            "Wheelchair" => SportType::Wheelchair,
            "Windsurf" => SportType::Windsurf,
            "Workout" => SportType::Workout,
            "Yoga" => SportType::Yoga,
            _ => SportType::Other(s.to_string()),
        })
    }
}

impl Serialize for SportType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SportType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        let Ok(sport_type) = value.parse();
        Ok(sport_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lists every known variant once, generating both the list the tests iterate over and an
    // exhaustive match, so adding a variant without updating this list fails to compile.
    macro_rules! all_variants {
        ($($variant:ident,)*) => {
            #[allow(dead_code)]
            fn exhaustive(sport_type: &SportType) {
                match sport_type {
                    $(SportType::$variant => {})*
                    SportType::Other(_) => {}
                }
            }

            const ALL: &[(SportType, &str)] = &[$((SportType::$variant, stringify!($variant)),)*];
        };
    }

    all_variants! {
        AlpineSki,
        BackcountrySki,
        Badminton,
        Basketball,
        Canoeing,
        Cricket,
        Crossfit,
        Dance,
        EBikeRide,
        Elliptical,
        EMountainBikeRide,
        Golf,
        GravelRide,
        Handcycle,
        HighIntensityIntervalTraining,
        Hike,
        IceSkate,
        InlineSkate,
        Kayaking,
        Kitesurf,
        MountainBikeRide,
        NordicSki,
        Padel,
        PhysicalTherapy,
        Pickleball,
        Pilates,
        Racquetball,
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
        Squash,
        StairStepper,
        StandUpPaddling,
        Surfing,
        Swim,
        TableTennis,
        Tennis,
        TrailRun,
        Velomobile,
        VirtualRide,
        VirtualRow,
        VirtualRun,
        Volleyball,
        Walk,
        WeightTraining,
        Wheelchair,
        Windsurf,
        Workout,
        Yoga,
    }

    #[test]
    fn as_str_matches_variant_name() {
        for (sport_type, name) in ALL {
            assert_eq!(sport_type.as_str(), *name);
            assert_eq!(sport_type.to_string(), *name);
        }
    }

    #[test]
    fn serde_round_trips_known_values() {
        for (sport_type, name) in ALL {
            let json = serde_json::to_string(sport_type).unwrap();
            assert_eq!(json, format!("\"{}\"", name));
            let parsed: SportType = serde_json::from_str(&json).unwrap();
            assert_eq!(&parsed, sport_type);
        }
    }

    #[test]
    fn keeps_unknown_values() {
        let parsed: SportType = serde_json::from_str(r#""Hurling""#).unwrap();
        assert_eq!(parsed, SportType::Other(String::from("Hurling")));
        assert_eq!(serde_json::to_string(&parsed).unwrap(), r#""Hurling""#);
    }
}

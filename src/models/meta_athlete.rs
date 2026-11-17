use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaAthlete {
    /// The unique identifier of the athlete
    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_64_bit_id() {
        crate::models::assert_round_trips::<MetaAthlete>(r#"{"id": 9007199254740993}"#);
    }
}

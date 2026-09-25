use serde::{Deserialize, Serialize};

/// The identifier of an activity, as nested in other objects
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaActivity {
    /// The unique identifier of the activity
    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_field() {
        crate::models::assert_round_trips::<MetaActivity>(r#"{"id": 12345678902}"#);
    }
}

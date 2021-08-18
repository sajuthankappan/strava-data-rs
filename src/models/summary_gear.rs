use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummaryGear {
  /// The gear's unique identifier.
  pub id: Option<String>,
  /// Resource state, indicates level of detail. Possible values: 2 -> \"summary\", 3 -> \"detail\"
  pub resource_state: Option<i32>,
  /// Whether this gear's is the owner's default one.
  pub primary: Option<bool>,
  /// The gear's name.
  pub name: Option<String>,
  /// The distance logged with this gear.
  pub distance: Option<f32>,
}

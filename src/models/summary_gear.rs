use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Serialize, Deserialize, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct SummaryGear {
  /// The gear's unique identifier.
  id: Option<String>,
  /// Resource state, indicates level of detail. Possible values: 2 -> \"summary\", 3 -> \"detail\"
  resource_state: Option<i32>,
  /// Whether this gear's is the owner's default one.
  primary: Option<bool>,
  /// The gear's name.
  name: Option<String>,
  /// The distance logged with this gear.
  distance: Option<f32>,
}

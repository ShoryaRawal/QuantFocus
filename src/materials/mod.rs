//! Materials subsystem root: predefined and custom material definitions.
pub mod presets;
pub mod custom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub atomic_number: u8,
    pub density_g_cm3: f64,
}

/// Retrieve a preset material by name (case-insensitive).
///
/// Returns a cloned `Material` if found, or `None` otherwise.
pub fn get_preset_material(name: &str) -> Option<Material> {
    presets::PRESETS
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(name))
        .cloned()
}

/// List all available preset material names.
pub fn list_preset_names() -> Vec<&'static str> {
    presets::PRESETS.iter().map(|m| m.name.as_str()).collect()
}

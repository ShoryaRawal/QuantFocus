//! Custom material creation and parsing from user input (e.g., JSON).

use serde::{Deserialize, Serialize};
use crate::materials::Material;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomMaterialSpec {
    /// Name of the material.
    pub name: String,
    /// Atomic number (Z) of the material. Must be between 1 and 100.
    pub atomic_number: u8,
    /// Density in g/cm³. Must be > 0.
    pub density_g_cm3: f64,
}

impl CustomMaterialSpec {
    pub fn try_into_material(self) -> Result<Material, String> {
        if self.name.trim().is_empty() {
            return Err("Material name cannot be empty".into());
        }
        if !(1..=100).contains(&self.atomic_number) {
            return Err(
                format!("atomic_number ({}) must be between 1 and 100", self.atomic_number)
            );
        }
        if self.density_g_cm3 <= 0.0 {
            return Err(
                format!("density_g_cm3 ({}) must be > 0", self.density_g_cm3)
            );
        }
        Ok(Material {
            name: self.name,
            atomic_number: self.atomic_number,
            density_g_cm3: self.density_g_cm3,
        })
    }
}

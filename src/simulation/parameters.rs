//! Defines the parameters for a single SEM simulation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub energy_kev: f64,
    pub current_na: f64,
    pub resolution: i32,
    pub distance_mm: f64,
}

impl SimulationParameters {
    pub fn new(
        energy_kev: f64,
        current_na: f64,
        resolution: i32,
        distance_mm: f64,
    ) -> Result<Self, String> {
        if !(1.0..=100.0).contains(&energy_kev) {
            return Err(format!("energy_kev ({} keV) out of range [1.0, 100.0]", energy_kev));
        }
        if current_na <= 0.0 {
            return Err(format!("current_na ({} nA) must be > 0", current_na));
        }
        if resolution <= 0 {
            return Err(format!("resolution ({}) must be > 0", resolution));
        }
        if distance_mm <= 0.0 {
            return Err(format!("distance_mm ({} mm) must be > 0", distance_mm));
        }

        Ok(Self {
            energy_kev,
            current_na,
            resolution,
            distance_mm,
        })
    }

    pub fn from_degrees(
        energy_kev: f64,
        current_na: f64,
        resolution: i32,
        distance_mm: f64,
    ) -> Result<Self, String> {
        Self::new(energy_kev, current_na, resolution, distance_mm)
    }
}

//! Defines the parameters for a single SEM simulation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub energy_kev: f64,
    pub thickness_nm: f64,
    pub angle_stddev_rad: f64,
    pub num_electrons: usize,
}

impl SimulationParameters {
    pub fn new(
        energy_kev: f64,
        thickness_nm: f64,
        angle_stddev_rad: f64,
        num_electrons: usize,
    ) -> Result<Self, String> {
        if !(1.0..=100.0).contains(&energy_kev) {
            return Err(format!("energy_kev ({} keV) out of range [1.0, 100.0]", energy_kev));
        }
        if thickness_nm <= 0.0 {
            return Err(format!("thickness_nm ({} nm) must be > 0", thickness_nm));
        }
        if angle_stddev_rad < 0.0 {
            return Err(format!("angle_stddev_rad ({}) must be ≥ 0", angle_stddev_rad));
        }
        if num_electrons == 0 {
            return Err("num_electrons must be ≥ 1".into());
        }

        Ok(Self {
            energy_kev,
            thickness_nm,
            angle_stddev_rad,
            num_electrons,
        })
    }

    pub fn from_degrees(
        energy_kev: f64,
        thickness_nm: f64,
        angle_stddev_deg: f64,
        num_electrons: usize,
    ) -> Result<Self, String> {
        let radians = angle_stddev_deg.to_radians();
        Self::new(energy_kev, thickness_nm, radians, num_electrons)
    }
}

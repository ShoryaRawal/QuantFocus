//! This module provides image formation from simulation data and export utilities.

pub mod formation;
pub mod export;

/// Lookup table type: mapping 0..=255 to new 0..=255 values
pub type Lut = [u8; 256];

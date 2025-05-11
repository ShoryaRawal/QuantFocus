pub mod ffi;
pub mod simulation;
pub mod materials;
pub mod imaging;

#[cfg(test)]
mod tests {
    use super::simulation::parameters::SimulationParameters;

    #[test]
    fn test_simulation_param_creation() {
        let params = SimulationParameters::new(20.0, 5.0, 1.0, 10000);
        assert_eq!(params.energy_kev, 20.0);
        assert_eq!(params.thickness_nm, 5.0);
        assert_eq!(params.angle_stddev_deg, 1.0);
        assert_eq!(params.num_electrons, 10000);
    }
}

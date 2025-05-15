pub mod ffi;
pub mod simulation;
pub mod materials;
pub mod imaging;

#[cfg(test)]
mod tests {
    use super::simulation::parameters::SimulationParameters;

    #[test]
    fn test_simulation_param_creation() {
        let params = SimulationParameters::new(20.0, 5.0, 256, 10.0);
        assert!(params.is_ok());
        let params = params.unwrap();
        assert_eq!(params.energy_kev, 20.0);
        assert_eq!(params.current_na, 5.0);
        assert_eq!(params.resolution, 256);
        assert_eq!(params.distance_mm, 10.0);
    }
}

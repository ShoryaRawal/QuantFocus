//! This module manages simulation jobs, parameter sweeps, and result collection.
pub mod parameters;
pub mod results;

use crate::ffi::wrapper::{init_simulation, run_simulation, get_scatter_data};
use parameters::SimulationParameters;
use results::SimulationResult;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Manages a queue of simulation jobs and executes them in parallel.
pub struct SimulationManager {
    /// Shared list of parameters for jobs
    jobs: Arc<Mutex<Vec<SimulationParameters>>>,
}

impl SimulationManager {
    /// Creates a new SimulationManager with an empty job queue.
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Enqueue a single simulation job with the given parameters.
    pub fn enqueue(&self, params: SimulationParameters) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(params);
    }

    /// Run all enqueued simulation jobs in parallel and return the results.
    pub fn run_all(&self) -> Vec<SimulationResult> {
        let jobs = {
            let mut locked = self.jobs.lock().unwrap();
            std::mem::take(&mut *locked)
        };

        // Execute simulations in parallel using Rayon
        jobs.into_par_iter()
            .map(|params| {
                // Initialize and run the Fortran simulation
                init_simulation(
                    params.energy_kev,
                    params.current_na,
                    params.resolution,
                    params.distance_mm,
                );
                run_simulation();

                // Retrieve raw scatter data
                let scatter = get_scatter_data();

                // Process into a SimulationResult
                SimulationResult::from_scatter(scatter, &params)
            })
            .collect()
    }

    /// Clears any pending jobs without running them.
    pub fn clear(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.clear();
    }
}

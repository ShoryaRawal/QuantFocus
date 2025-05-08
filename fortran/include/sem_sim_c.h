/**
 * @file sem_sim_c.h
 * @brief C interface for the Scanning Electron Microscope Simulator
 * 
 * This header defines the C-compatible interface for the Fortran-based SEM simulator.
 * It provides structures and functions that can be called from Rust, C, or other languages
 * that support C FFI.
 */

#ifndef SEM_SIM_C_H
#define SEM_SIM_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/**
 * @brief Error codes returned by simulator functions
 */
typedef enum {
    SEM_SUCCESS = 0,             /**< Operation completed successfully */
    SEM_ERR_INVALID_PARAM = -1,  /**< Invalid parameter */
    SEM_ERR_MEMORY = -2,         /**< Memory allocation error */
    SEM_ERR_PHYSICS = -3,        /**< Physics calculation error */
    SEM_ERR_IO = -4,             /**< Input/output error */
    SEM_ERR_NOT_INITIALIZED = -5 /**< Simulator not initialized */
} sem_error_t;

/**
 * @brief Signal types that can be detected from the sample
 */
typedef enum {
    SEM_SIGNAL_SECONDARY = 0,  /**< Secondary electrons */
    SEM_SIGNAL_BACKSCATTERED,  /**< Backscattered electrons */
    SEM_SIGNAL_COMBINED        /**< Combined signal */
} sem_signal_type_t;

/**
 * @brief Noise model types
 */
typedef enum {
    SEM_NOISE_NONE = 0,    /**< No noise */
    SEM_NOISE_POISSON,     /**< Poisson noise (quantum) */
    SEM_NOISE_GAUSSIAN,    /**< Gaussian noise (electronic) */
    SEM_NOISE_COMBINED     /**< Combined Poisson and Gaussian noise */
} sem_noise_model_t;

/**
 * @brief Material definition structure
 */
typedef struct {
    double atomic_number;     /**< Effective atomic number (Z) */
    double density;           /**< Density in g/cm³ */
    double work_function;     /**< Work function in eV */
    double mean_ionization;   /**< Mean ionization energy in eV */
    const char* name;         /**< Material name */
} sem_material_t;

/**
 * @brief Electron beam parameters
 */
typedef struct {
    double energy_kev;        /**< Beam energy in keV */
    double current_na;        /**< Beam current in nA */
    double working_distance;  /**< Working distance in mm */
    double spot_size;         /**< Spot size in nm */
    double convergence_angle; /**< Beam convergence angle in radians */
} sem_beam_params_t;

/**
 * @brief Scan parameters
 */
typedef struct {
    uint32_t width;           /**< Image width in pixels */
    uint32_t height;          /**< Image height in pixels */
    double pixel_size;        /**< Pixel size in nm */
    double dwell_time;        /**< Dwell time per pixel in μs */
    uint32_t line_avg;        /**< Line averaging count */
    uint32_t frame_avg;       /**< Frame averaging count */
} sem_scan_params_t;

/**
 * @brief Monte Carlo simulation parameters
 */
typedef struct {
    uint32_t num_electrons;       /**< Number of electrons per pixel */
    uint32_t max_collisions;      /**< Maximum collisions per electron */
    double min_energy;            /**< Minimum tracking energy in eV */
    double max_depth;             /**< Maximum tracking depth in nm */
    bool track_secondaries;       /**< Whether to track secondary electrons */
} sem_mc_params_t;

/**
 * @brief Detector parameters
 */
typedef struct {
    sem_signal_type_t signal_type;    /**< Type of signal to detect */
    double collection_efficiency;     /**< Detector collection efficiency (0-1) */
    double energy_threshold;          /**< Detection energy threshold in eV */
    double take_off_angle;            /**< Detector take-off angle in radians */
    double azimuthal_angle;           /**< Detector azimuthal angle in radians */
    sem_noise_model_t noise_model;    /**< Noise model to apply */
    double noise_param1;              /**< First noise parameter (SNR for Gaussian) */
    double noise_param2;              /**< Second noise parameter (std for Gaussian) */
} sem_detector_params_t;

/**
 * @brief Image processing parameters
 */
typedef struct {
    double brightness;     /**< Brightness adjustment (-1.0 to 1.0) */
    double contrast;       /**< Contrast adjustment (0.0 to 2.0) */
    double gamma;          /**< Gamma correction (typically 0.4 to 2.5) */
} sem_image_params_t;

/**
 * @brief Complete simulation parameters
 */
typedef struct {
    sem_beam_params_t beam;           /**< Electron beam parameters */
    sem_scan_params_t scan;           /**< Scan parameters */
    sem_mc_params_t monte_carlo;      /**< Monte Carlo parameters */
    sem_detector_params_t detector;   /**< Detector parameters */
    sem_image_params_t image;         /**< Image processing parameters */
} sem_sim_params_t;

/**
 * @brief Sample structure definition
 */
typedef struct {
    uint32_t id;               /**< Sample ID */
    void* internal_ptr;        /**< Internal pointer to Fortran data (opaque) */
} sem_sample_t;

/**
 * @brief Simulation context handle
 */
typedef struct {
    uint32_t id;               /**< Simulation ID */
    void* internal_ptr;        /**< Internal pointer to Fortran data (opaque) */
} sem_sim_ctx_t;

/**
 * @brief Image data structure
 */
typedef struct {
    uint32_t width;            /**< Image width in pixels */
    uint32_t height;           /**< Image height in pixels */
    double* data;              /**< Image data, row-major order, dynamically allocated */
    uint8_t* data_8bit;        /**< 8-bit image data for display, dynamically allocated */
} sem_image_t;

/**
 * @brief Simulation results structure
 */
typedef struct {
    sem_image_t image;                 /**< Simulated SEM image */
    double avg_penetration_depth;      /**< Average electron penetration depth in nm */
    double backscatter_coefficient;    /**< Backscatter coefficient (0-1) */
    uint64_t total_electrons_simulated;/**< Total electrons simulated */
    double simulation_time_s;          /**< Simulation time in seconds */
} sem_sim_results_t;

/**
 * @brief Initialize the SEM simulator library
 * 
 * Must be called before any other functions
 * 
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_initialize(void);

/**
 * @brief Finalize the SEM simulator library
 * 
 * Must be called when done using the simulator to free resources
 * 
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_finalize(void);

/**
 * @brief Get the library version
 * 
 * @param major Pointer to store major version
 * @param minor Pointer to store minor version
 * @param patch Pointer to store patch version
 * @return SEM_SUCCESS on success
 */
sem_error_t sem_get_version(int* major, int* minor, int* patch);

/**
 * @brief Create a new simulation context
 * 
 * @param[out] ctx Pointer to store the created context
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_create_context(sem_sim_ctx_t* ctx);

/**
 * @brief Destroy a simulation context
 * 
 * @param ctx Context to destroy
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_destroy_context(sem_sim_ctx_t ctx);

/**
 * @brief Create a homogeneous material sample
 * 
 * @param ctx Simulation context
 * @param material Material definition
 * @param width Width of the sample in nm
 * @param height Height of the sample in nm
 * @param depth Depth of the sample in nm
 * @param[out] sample Pointer to store the created sample
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_create_homogeneous_sample(
    sem_sim_ctx_t ctx,
    sem_material_t material,
    double width,
    double height,
    double depth,
    sem_sample_t* sample
);

/**
 * @brief Create a layered material sample
 * 
 * @param ctx Simulation context
 * @param materials Array of material definitions
 * @param thicknesses Array of layer thicknesses in nm
 * @param num_layers Number of layers
 * @param width Width of the sample in nm
 * @param height Height of the sample in nm
 * @param[out] sample Pointer to store the created sample
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_create_layered_sample(
    sem_sim_ctx_t ctx,
    sem_material_t* materials,
    double* thicknesses,
    uint32_t num_layers,
    double width,
    double height,
    sem_sample_t* sample
);

/**
 * @brief Destroy a sample
 * 
 * @param ctx Simulation context
 * @param sample Sample to destroy
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_destroy_sample(sem_sim_ctx_t ctx, sem_sample_t sample);

/**
 * @brief Get a predefined material by name
 * 
 * @param name Material name (e.g., "Cu", "Si", "C")
 * @param[out] material Pointer to store the material definition
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_get_predefined_material(const char* name, sem_material_t* material);

/**
 * @brief Run a complete SEM simulation
 * 
 * @param ctx Simulation context
 * @param sample Sample to simulate
 * @param params Simulation parameters
 * @param[out] results Pointer to store simulation results
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_run_simulation(
    sem_sim_ctx_t ctx,
    sem_sample_t sample,
    sem_sim_params_t params,
    sem_sim_results_t* results
);

/**
 * @brief Clean up simulation results
 * 
 * Frees dynamically allocated memory in results
 * 
 * @param results Results to clean up
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_cleanup_results(sem_sim_results_t* results);

/**
 * @brief Save simulation image to a file
 * 
 * @param results Simulation results containing the image
 * @param filename Output filename
 * @param format Format string ("png", "bmp", "raw")
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_save_image(sem_sim_results_t results, const char* filename, const char* format);

/**
 * @brief Get error message for an error code
 * 
 * @param error Error code
 * @return String description of the error
 */
const char* sem_get_error_message(sem_error_t error);

/**
 * @brief Get default simulation parameters
 * 
 * Fills the params structure with reasonable defaults
 * 
 * @param[out] params Pointer to parameter structure to fill
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_get_default_params(sem_sim_params_t* params);

/**
 * @brief Initialize Monte Carlo engine
 * 
 * Called internally by sem_run_simulation, but exposed for advanced use
 * 
 * @param ctx Simulation context
 * @param params Monte Carlo parameters
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_init_monte_carlo(sem_sim_ctx_t ctx, sem_mc_params_t params);

/**
 * @brief Simulate a single electron trajectory
 * 
 * Advanced function for custom simulation loops
 * 
 * @param ctx Simulation context
 * @param sample Sample to simulate
 * @param energy_kev Initial electron energy in keV
 * @param x Initial x position in nm
 * @param y Initial y position in nm
 * @param z Initial z position in nm
 * @param dx Initial x direction
 * @param dy Initial y direction
 * @param dz Initial z direction
 * @param[out] bse_detected Whether a backscattered electron was detected
 * @param[out] se_count Number of secondary electrons detected
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_simulate_electron(
    sem_sim_ctx_t ctx,
    sem_sample_t sample,
    double energy_kev,
    double x, double y, double z,
    double dx, double dy, double dz,
    bool* bse_detected,
    uint32_t* se_count
);

/**
 * @brief Set a callback function for progress reporting
 * 
 * @param ctx Simulation context
 * @param callback Function pointer to callback (takes double progress 0-1)
 * @param user_data User data pointer passed to callback
 * @return SEM_SUCCESS on success, error code otherwise
 */
sem_error_t sem_set_progress_callback(
    sem_sim_ctx_t ctx,
    void (*callback)(double progress, void* user_data),
    void* user_data
);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // SEM_SIM_C_H

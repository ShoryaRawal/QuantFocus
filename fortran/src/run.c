#include "sem_sim_c.h"
#include <stddef.h>

// Forward declarations of Fortran functions with correct names
extern void init_simulation(double* energy, double* current, int* resolution, double* distance);
extern void run_simulation(void);
extern void fortran_get_image_data(double** data, int* width, int* height);

// C wrapper functions that match the header declarations
void c_init_simulation(double energy, double current, int resolution, double distance) {
    init_simulation(&energy, &current, &resolution, &distance);
}

void c_run_simulation(void) {
    run_simulation();
}

void c_get_image_data(double** data, int* width, int* height) {
    fortran_get_image_data(data, width, height);
}

#ifndef SEM_SIM_C_H
#define SEM_SIM_C_H

#ifdef __cplusplus
extern "C" {
#endif

void c_init_simulation(double energy, double thickness, double angle_stddev, int n_electrons);
void c_run_simulation(void);
void c_get_scatter_data(double** data, int* rows, int* cols);

#ifdef __cplusplus
}
#endif

#endif

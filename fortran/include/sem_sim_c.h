#ifndef SEM_SIM_C_H
#define SEM_SIM_C_H

#ifdef __cplusplus
extern "C" {
#endif

void c_init_simulation(double energy, double current, int resolution, double distance);
void c_run_simulation(void);
void c_get_scatter_data(double** data, int* rows, int* cols);
void c_get_line_data(double** data, int* points);
void c_get_image_data(double** data, int* width, int* height);

#ifdef __cplusplus
}
#endif

#endif

#include <stdio.h>

// Declare Fortran subroutines exposed via bind(C)

extern void c_init_simulation(double *energy, double *thickness, double *angle_stddev, int *num_electrons);
extern void c_run_simulation(void);
extern void c_get_scatter_data(double **data, int *rows, int *cols);

int main() {
    double energy = 20.0;
    double thickness = 500.0;
    double angle_stddev = 0.01;
    int electrons = 1000;
    
    c_init_simulation(&energy, &thickness, &angle_stddev, &electrons);
    
    printf("Simulation initialized.\n");

    // Run simulation
    c_run_simulation();
    printf("Simulation completed.\n");

    // Retrieve data
    double *data = NULL;
    int rows, cols;
    c_get_scatter_data(&data, &rows, &cols);

    printf("Scatter data (first 5 electrons):\n");
    for (int i = 0; i < 5 && i < cols; ++i) {
        printf("Electron %d: x=%f y=%f z=%f\n", i + 1,
               data[i * rows + 0],
               data[i * rows + 1],
               data[i * rows + 2]);
    }

    return 0;
}

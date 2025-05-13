# Compiler settings
FC = gfortran
CC = gcc
FFLAGS = -c -O2
CFLAGS = -c -O2
LDFLAGS =

# Files
F90_SRC = beam.f90 materials.f90 scattering.f90 signals.f90 monte_carlo.f90 c_interface.f90
C_SRC = run.c

F90_OBJ = $(F90_SRC:.f90=.o)
C_OBJ = $(C_SRC:.c=.o)

# Executable name
TARGET = sim

# Default target
all: $(TARGET)

# Link the final executable
$(TARGET): $(F90_OBJ) $(C_OBJ)
	$(FC) $(F90_OBJ) $(C_OBJ) -o $(TARGET)

# Compile Fortran sources
%.o: %.f90
	$(FC) $(FFLAGS) $< -o $@

# Compile C sources
%.o: %.c
	$(CC) $(CFLAGS) $< -o $@

# Clean up build artifacts
clean:
	rm -f *.o *.mod $(TARGET)

# Run the simulation
run: $(TARGET)
	./$(TARGET)


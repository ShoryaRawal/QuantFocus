# Makefile for building Fortran static library (libsem_sim.a)

FC = gfortran
SRC_DIR = fortran/src
BUILD_DIR = fortran/build
INCLUDE_DIR = fortran/include
OBJS = $(patsubst $(SRC_DIR)/%.f90, $(BUILD_DIR)/%.o, $(wildcard $(SRC_DIR)/*.f90))
LIB = $(BUILD_DIR)/libsem_sim.a

# Flags
FFLAGS = -O2 -fPIC -Wall -std=f2008

all: $(LIB)

# Compile Fortran source files into .o files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.f90 | $(BUILD_DIR)
	$(FC) $(FFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

# Create static library from .o files
$(LIB): $(OBJS)
	ar rcs $@ $^

# Ensure build directory exists
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean

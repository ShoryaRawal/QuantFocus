cmake_minimum_required(VERSION 3.15)
project(sem_sim LANGUAGES Fortran)

# === Options ===
option(BUILD_SHARED_LIBS "Build shared instead of static libraries" OFF)
option(ENABLE_TESTS "Build Fortran unit tests" OFF)

# === Include paths ===
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# === Fortran sources ===
set(SEM_SIM_SOURCES
    src/beam.f90
    src/monte_carlo.f90
    src/materials.f90
    src/scattering.f90
    src/signals.f90
    src/c_interface.f90
)

# === Create the library ===
add_library(sem_sim ${SEM_SIM_SOURCES})

# Require Fortran 2003 standard
set_target_properties(sem_sim PROPERTIES
    Fortran_STANDARD 2003
    Fortran_STANDARD_REQUIRED ON
)

# === C header installation ===
install(FILES include/sem_sim_c.h DESTINATION include)

# === Library installation ===
install(TARGETS sem_sim
        EXPORT sem_sim_targets
        ARCHIVE DESTINATION lib
        LIBRARY DESTINATION lib
        RUNTIME DESTINATION bin)

# === Export targets ===
install(EXPORT sem_sim_targets
        FILE sem_simTargets.cmake
        NAMESPACE sem_sim::
        DESTINATION lib/cmake/sem_sim)

# === Tests ===
if(ENABLE_TESTS)
    enable_testing()
    # Add test executables only if sources exist
    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/tests/test_beam.f90)
        add_executable(test_beam tests/test_beam.f90)
        target_link_libraries(test_beam PRIVATE sem_sim)
        add_test(NAME beam_test COMMAND test_beam)
    endif()
    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/tests/test_monte_carlo.f90)
        add_executable(test_monte_carlo tests/test_monte_carlo.f90)
        target_link_libraries(test_monte_carlo PRIVATE sem_sim)
        add_test(NAME monte_carlo_test COMMAND test_monte_carlo)
    endif()
endif()

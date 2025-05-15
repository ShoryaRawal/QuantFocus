use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Trigger rebuild if C header or source changes
    println!("cargo:rerun-if-changed=fortran/include/sem_sim_c.h");
    println!("cargo:rerun-if-changed=fortran/src/run.c");
    println!("cargo:rerun-if-changed=fortran/src/monte_carlo.f90");
    println!("cargo:rerun-if-changed=fortran/src/c_interface.f90");
    println!("cargo:rerun-if-changed=fortran/src/signals.f90");

    // === Generate Rust bindings for the C interface ===
    let bindings = bindgen::Builder::default()
        .header("fortran/include/sem_sim_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .allowlist_function("c_.*") // Only include the c_ prefixed functions
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // === Compile C wrapper ===
    cc::Build::new()
        .file("fortran/src/run.c")
        .include("fortran/include")
        .compile("sem_c_api");

    // === Link against Fortran libraries ===
    println!("cargo:rustc-link-search=native=fortran/build");
    println!("cargo:rustc-link-lib=static=sem_sim");
    
    // Link against the Fortran runtime (GFortran)
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/gcc/14.2.0_1/lib/gcc/current");
    println!("cargo:rustc-link-lib=gfortran");
    
    // Link against system libraries
    println!("cargo:rustc-link-lib=dylib=System");
    println!("cargo:rustc-link-lib=dylib=c");
    println!("cargo:rustc-link-lib=dylib=m");
}

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Trigger rebuild if C header changes
    let header_path = "fortran/include/sem_sim_c.h";
    println!("cargo:rerun-if-changed={}", header_path);

    // === Generate Rust bindings for the C interface ===
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rust_target(bindgen::RustTarget::Nightly)
        .wrap_unsafe_ops(true)
        .generate_comments(true)
        .allowlist_function("c_.*") // Only include the c_ prefixed functions
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // === Compile C wrapper if needed ===
    cc::Build::new()
        .file("fortran/src/run.c")
        .include("fortran/include")
        .compile("sem_c_api");

    // === Link Fortran static library ===
    let fortran_lib_path = Path::new("fortran/build");
    let fortran_lib_file = fortran_lib_path.join("libsem_sim.a");

    if !fortran_lib_file.exists() {
        panic!(
            "Fortran static library not found at {}. Please run `cmake .. && make` in `fortran/build` first.",
            fortran_lib_file.display()
        );
    }

    // Tell Cargo where to find and how to link
    println!("cargo:rustc-link-search=native={}", fortran_lib_path.display());
    println!("cargo:rustc-link-lib=static=sem_sim");     // Link to Fortran lib
    println!("cargo:rustc-link-lib=static=sem_c_api");    // Link to C wrapper

    // Link against the Fortran runtime (GFortran)
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/gcc/14.2.0_1/lib/gcc/current");
    println!("cargo:rustc-link-lib=gfortran");
}

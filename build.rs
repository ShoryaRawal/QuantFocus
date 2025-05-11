// build.rs

use std::env;
use std::path::PathBuf;

// fn main() {
//     let bindings = bindgen::Builder::default()
//         .header("fortran/include/sem_sim_c.h") // Change to your actual header
//         .generate()
//         .expect("Unable to generate bindings");

//     let out_path = std::env::var("OUT_DIR").unwrap();
//     bindings
//         .write_to_file(std::path::Path::new(&out_path).join("bindings.rs"))
//         .expect("Couldn't write bindings!");
// }

fn main() {
    let bindings = bindgen::Builder::default()
        .header("fortran/include/sem_sim_c.h")
        .rust_target(bindgen::RustTarget::Nightly)
        .wrap_unsafe_ops(true) 
        .generate()      
        .expect("Unable to generate bindings");

    let out_path = std::env::var("OUT_DIR").unwrap();
    bindings
        .write_to_file(std::path::Path::new(&out_path).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("fortran/src/run.c")
        .include("fortran/include")
        .compile("sem_c_api");
}
// fn main() {
//     let header_path = "fortran/include/sem_sim_c.h";
//     println!("cargo:rerun-if-changed={}", header_path);

//     let bindings = bindgen::Builder::default()
//         .header(header_path)
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .generate_comments(true)
//         .allowlist_function("c_.*")
//         .generate()
//         .expect("Unable to generate bindings");

//     // Write to $OUT_DIR/bindings.rs
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
//     bindings
//         .write_to_file(out_path)
//         .expect("Couldn't write bindings to $OUT_DIR");
// }

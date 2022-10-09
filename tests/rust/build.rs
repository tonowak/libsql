extern crate bindgen;

fn main() {
    // Tell cargo to tell rustc to link the lib
    println!("cargo:rustc-link-search=native={}", "../../");
    println!("cargo:rustc-link-lib=sqlite3");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=../../sqlite3.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../../sqlite3.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the bindings.rs file.
    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}

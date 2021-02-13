use std::env;
use std::path::PathBuf;

// TODO Make the optional modules....optional

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    cc::Build::new()
        .files(glob::glob("vm/*.c").expect("Failed to glob c files").filter_map(|x| x.ok()))
        .files(glob::glob("optional/*.c").expect("Failed to glob c files").filter_map(|x| x.ok()))
        .include("include")
        .include("optional")
        .include("vm")
        .compile("wren");

    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .header("wrapper.h")
        .whitelist_var("WREN.*")
        .whitelist_type("Wren.*")
        .whitelist_function("wren.*")
        .clang_arg("-Ivm")
        .clang_arg("-Iinclude")
        .clang_arg("-Ioptional")
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default()
        .header_contents("bindings.h", "#include <rcl/rcl.h>")
        .derive_copy(false)
        .default_enum_style(bindgen::EnumVariation::Rust)
        .clang_arg("-I/opt/ros/foxy/include");

    println!("cargo:rustc-link-search=native=/opt/ros/foxy/lib");
    println!("cargo:rustc-link-lib=dylib=rcl");

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default()
        .header_contents("bindings.h", "#include <geometry_msgs/msg/vector3.h>")
        .clang_arg("-I../../../include")
        .clang_arg("-I/opt/ros/foxy/include");

    println!("cargo:rustc-link-search=native=../../../lib");
    println!("cargo:rustc-link-lib=dylib=geometry_msgs__rosidl_typesupport_c");

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

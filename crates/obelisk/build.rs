extern crate cxx_build;

fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("src/include")
        .include("vendor/node/src")
        .include("vendor/node/deps/v8/include")
        .flag_if_supported("-std=c++11")
        .compile("shim");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=vendor/node/src/node.h");
    println!("cargo:rerun-if-changed=vendor/node/deps/v8/include/v8.h");
}

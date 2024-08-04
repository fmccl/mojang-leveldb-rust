extern crate bindgen;
mod build_leveldb;
mod build_zlib;

use std::{env, path::PathBuf};

fn main() {
    build_zlib::build_zlib();

    build_leveldb::build_leveldb();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg("-Ileveldb/include/")
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let binding_text = std::fs::read_to_string(out_path.join("bindings.rs"))
        .expect("Failed to read generated bindings");

    std::fs::write(
        out_path.join("bindings.rs"),
        format!(
            "#![allow(non_camel_case_types)]\n\
    #![allow(non_snake_case)]\n\
    #![allow(non_upper_case_globals)]\n\
    #![allow(dead_code)]\n\
    {}\n",
            binding_text
        ),
    )
    .expect("Failed to write to generated bindings");

    // Link object files using cc crate
    // println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=leveldb");
    if env::consts::FAMILY != "windows" {
        println!("cargo:rustc-link-lib=stdc++");
    } else {
        println!("cargo:rustc-link-lib=msvcrt");
    }
    println!("cargo:rustc-link-lib=static=zlib");
}

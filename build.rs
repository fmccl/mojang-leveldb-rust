extern crate bindgen;

use std::{env, path::PathBuf};

fn main() {
    let zlib_path = cmake::build("zlib");

    let mut build = cmake::Config::new("leveldb");
    build.cxxflag("-I".to_string() + &zlib_path.display().to_string() + "/include");
    let dst = build.build();


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

    std::fs::write(out_path.join("bindings.rs"), format!("#![allow(non_camel_case_types)]\n\
    #![allow(non_snake_case)]\n\
    #![allow(non_upper_case_globals)]\n\
    #![allow(dead_code)]\n\
    {}\n", binding_text)).expect("Failed to write to generated bindings");

    let debug = env::var("PROFILE").unwrap() == "debug";

    // Link object files using cc crate
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", zlib_path.display());
    println!("cargo:rustc-link-lib=static=leveldb");
    if env::consts::FAMILY != "windows" {
        println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=static=z");

    } else {
        if debug {
            println!("cargo:rustc-link-lib=static=zlibstaticd");
        } else {
            println!("cargo:rustc-link-lib=static=zlibstatic");
        }
        println!("cargo:rustc-link-lib=msvcrt");
    }

}

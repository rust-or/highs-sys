use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let mut dst = Config::new("HiGHS");

    if cfg!(feature = "ninja") {
        dst.generator("Ninja");
    }

    // Avoid using downstream project's profile setting for HiGHS build.
    if cfg!(feature = "highs_release") {
        dst.profile("Release");
    }

    let dst = dst
        .define("FAST_BUILD", "ON")
        .define("SHARED", "OFF")
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL")
        .define("CMAKE_INTERPROCEDURAL_OPTIMIZATION", "FALSE")
        .build();

    let include_path = dst.join("include").join("highs");
    let src_path = PathBuf::from("HiGHS").join("src");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let c_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(
            include_path
                .join("interfaces")
                .join("highs_c_api.h")
                .to_string_lossy(),
        )
        .clang_args(&[
            &format!("-I{}", include_path.to_string_lossy()),
            &format!("-I{}", src_path.to_string_lossy()),
        ])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    c_bindings
        .write_to_file(out_path.join("c_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=highs");

    if cfg!(feature = "libz") {
        println!("cargo:rustc-link-lib=z");
    }

    let target = env::var("TARGET").unwrap();
    let apple = target.contains("apple");
    let windows = target.contains("windows");
    let linux = target.contains("linux");
    if apple {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if linux {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    if apple {
        println!("cargo:rustc-link-lib=dylib=omp");
    } else if !windows {
        // No openmp 3 on windows
        println!("cargo:rustc-link-lib=dylib=gomp");
    }
    println!("cargo:rerun-if-changed=HiGHS/src/interfaces/highs_c_api.h");
}

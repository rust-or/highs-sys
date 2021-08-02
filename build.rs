use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // Targets
    let target = env::var("TARGET").unwrap();
    let apple = target.contains("apple");
    let windows = target.contains("windows");
    let linux = target.contains("linux");
    let musl = target.contains("musl");

    let mut dst = Config::new("HiGHS");
    dst.define("FAST_BUILD", "ON")
        .define("SHARED", "OFF")
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");

    if musl {
        dst.define(
            "CMAKE_CXX_STANDARD_LIBRARIES",
            "-static-libgcc -static-libstdc++",
        );
    };

    let dst = dst.build();
    let include_path = dst.join("include");
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
    println!("cargo:rustc-link-lib=static=highs");

    if apple {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if musl {
        println!("cargo:rustc-link-lib=static=stdc++");
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

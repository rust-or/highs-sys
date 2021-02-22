use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("HiGHS").define("OPENMP", "ON").build();
    println!("cargo:rustc-link-search=all={}/lib", dst.display());
    println!("cargo:rustc-link-lib=highs");

    let include_path = dst.join("include");
    let src_path = PathBuf::from("HiGHS").join("src");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
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
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

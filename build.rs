use std::env;
use std::path::PathBuf;

fn main() {
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sdk_path = crate_path.join("sdk");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // https://stackoverflow.com/questions/41917096/how-do-i-make-rustc-link-search-relative-to-the-project-location
    let lib_path = sdk_path.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    let linker_script_path = sdk_path.join("ld").join("eagle.app.v6.ld");
    // let linker_script_path = sdk_path.join("ld").join("eagle.rom.addr.v6.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_script_path.display());

    // println!("cargo:rustc-link-lib=static=smartconfig");
    // println!("cargo:rustc-link-lib=static=pwm");
    // println!("cargo:rustc-link-lib=static=ssl");
    // println!("cargo:rustc-link-lib=static=upgrade");
    // println!("cargo:rustc-link-lib=static=json");
    println!("cargo:rustc-link-lib=static=main");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=wpa");
    println!("cargo:rustc-link-lib=static=lwip");
    println!("cargo:rustc-link-lib=static=net80211");
    println!("cargo:rustc-link-lib=static=pp");
    println!("cargo:rustc-link-lib=static=phy");
    // println!("cargo:rustc-link-lib=static=hal");
    // println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=c");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=sdk/include/*.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let local_include_path = crate_path.join("include");
    let builder = bindgen::Builder::default()
        .clang_arg(format!("-F{}", local_include_path.display()))
        .clang_arg(format!("--target={}", env::var("HOST").unwrap()))
        .clang_arg("--trace-includes") // TODO
        .use_core()
        .ctypes_prefix("cty")
        // The input header we would like to generate
        // bindings for.
        .header("sdk/include/user_interface.h")
        // .header("sdk/include/osapi.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false);

    let bindings = builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

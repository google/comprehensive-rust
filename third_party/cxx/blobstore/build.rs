fn main() {
    // Find target directory, either from CARGO_TARGET_DIR or in-tree if unset.
    let mut src_dir =
        std::env::var_os("CARGO_TARGET_DIR").unwrap_or("../../../target".into());
    src_dir.push("/cxxbridge/demo/src");

    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .flag_if_supported("-std=c++14")
        .include(".")
        .include(src_dir)
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}

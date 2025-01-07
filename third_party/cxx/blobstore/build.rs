fn main() {
    let src_dir = {
        let mut target_dir =
            std::env::var_os("CARGO_TARGET_DIR").unwrap_or("../../../target".into());
        target_dir.push("/cxxbridge/demo/src");
        target_dir
    };

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

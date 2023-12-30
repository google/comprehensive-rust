# Android

To build a bare-metal Rust binary in AOSP, you need to use a `rust_ffi_static`
Soong rule to build your Rust code, then a `cc_binary` with a linker script to
produce the binary itself, and then a `raw_binary` to convert the ELF to a raw
binary ready to be run.

<!-- mdbook-xgettext: skip -->

```soong
rust_ffi_static {
    name: "libvmbase_example",
    defaults: ["vmbase_ffi_defaults"],
    crate_name: "vmbase_example",
    srcs: ["src/main.rs"],
    rustlibs: [
        "libvmbase",
    ],
}

cc_binary {
    name: "vmbase_example",
    defaults: ["vmbase_elf_defaults"],
    srcs: [
        "idmap.S",
    ],
    static_libs: [
        "libvmbase_example",
    ],
    linker_scripts: [
        "image.ld",
        ":vmbase_sections",
    ],
}

raw_binary {
    name: "vmbase_example_bin",
    stem: "vmbase_example.bin",
    src: ":vmbase_example",
    enabled: false,
    target: {
        android_arm64: {
            enabled: true,
        },
    },
}
```

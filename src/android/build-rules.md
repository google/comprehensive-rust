# Build Rules

The Android build system (Soong) supports Rust via a number of modules:

| Module Type       | Description                                                                                        |
| ----------------- | -------------------------------------------------------------------------------------------------- |
| `rust_binary`     | Produces a Rust binary.                                                                            |
| `rust_library`    | Produces a Rust library, and provides both `rlib` and `dylib` variants.                            |
| `rust_ffi`        | Produces a Rust C library usable by `cc` modules, and provides both static and shared variants.    |
| `rust_proc_macro` | Produces a `proc-macro` Rust library. These are analogous to compiler plugins.                     |
| `rust_test`       | Produces a Rust test binary that uses the standard Rust test harness.                              |
| `rust_fuzz`       | Produces a Rust fuzz binary leveraging `libfuzzer`.                                                |
| `rust_protobuf`   | Generates source and produces a Rust library that provides an interface for a particular protobuf. |
| `rust_bindgen`    | Generates source and produces a Rust library containing Rust bindings to C libraries.              |

We will look at `rust_binary` and `rust_library` next.

<details>

Additional items speaker may mention:

- Cargo is not optimized for multi-language repos, and also downloads packages
  from the internet.

- For compliance and performance, Android must have crates in-tree. It must also
  interop with C/C++/Java code. Soong fills that gap.

- Soong has many similarities to [Bazel](https://bazel.build/), which is the
  open-source variant of Blaze (used in google3).

- Fun fact: Data from Star Trek is a Soong-type Android.

</details>

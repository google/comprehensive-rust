# Building in Android

Create two genrules: One to generate the CXX header, and one to generate the CXX
source file. These are then used as inputs to the `cc_library_static`.

```javascript
// Generate a C++ header containing the C++ bindings
// to the Rust exported functions in lib.rs.
genrule {
    name: "libcxx_test_bridge_header",
    tools: ["cxxbridge"],
    cmd: "$(location cxxbridge) $(in) --header > $(out)",
    srcs: ["lib.rs"],
    out: ["lib.rs.h"],
}

// Generate the C++ code that Rust calls into.
genrule {
    name: "libcxx_test_bridge_code",
    tools: ["cxxbridge"],
    cmd: "$(location cxxbridge) $(in) > $(out)",
    srcs: ["lib.rs"],
    out: ["lib.rs.cc"],
}
```

<details>

- The `cxxbridge` tool is a standalone tool that generates the C++ side of the
  bridge module. It is included in Android and available as a Soong tool.
- By convention, if your Rust source file is `lib.rs` your header file will be
  named `lib.rs.h` and your source file will be named `lib.rs.cc`. This naming
  convention isn't enforced, though.

</details>

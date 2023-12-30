# Building in Android

Create a `cc_library_static` to build the C++ library, including the CXX
generated header and source file.

```javascript
cc_library_static {
    name: "libcxx_test_cpp",
    srcs: ["cxx_test.cpp"],
    generated_headers: [
        "cxx-bridge-header",
        "libcxx_test_bridge_header"
    ],
    generated_sources: ["libcxx_test_bridge_code"],
}
```

<details>

- Point out that `libcxx_test_bridge_header` and `libcxx_test_bridge_code` are
  the dependencies for the CXX-generated C++ bindings. We'll show how these are
  setup on the next slide.
- Note that you also need to depend on the `cxx-bridge-header` library in order
  to pull in common CXX definitions.
- Full docs for using CXX in Android can be found in [the Android docs]. You may
  want to share that link with the class so that students know where they can
  find these instructions again in the future.

[the Android docs]: https://source.android.com/docs/setup/build/rust/building-rust-modules/android-rust-patterns#rust-cpp-interop-using-cxx

</details>

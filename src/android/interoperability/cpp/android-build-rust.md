# Building in Android

Create a `rust_binary` that depends on `libcxx` and your `cc_library_static`.

```javascript
rust_binary {
    name: "cxx_test",
    srcs: ["lib.rs"],
    rustlibs: ["libcxx"],
    static_libs: ["libcxx_test_cpp"],
}
```

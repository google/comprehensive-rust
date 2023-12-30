# Depending on a Crate

Once you've added a third-party crate and generated build rules, depending on a
crate is simple. Find your `rust_static_library` target, and add a `dep` on the
`:lib` target within your crate.

Specifically,

```bob
                     +------------+      +----------------------+
"//third_party/rust" | crate name | "/v" | major semver version | ":lib"
                     +------------+      +----------------------+
```

For instance,

```gn
rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
  deps = [ "//third_party/rust/example_rust_crate/v1:lib" ]
}
```

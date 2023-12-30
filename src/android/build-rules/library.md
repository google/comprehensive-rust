# Rust Libraries

You use `rust_library` to create a new Rust library for Android.

Here we declare a dependency on two libraries:

- `libgreeting`, which we define below,
- `libtextwrap`, which is a crate already vendored in
  [`external/rust/crates/`][crates].

[crates]: https://cs.android.com/android/platform/superproject/+/master:external/rust/crates/

_hello_rust/Android.bp_:

```javascript
{{#include library/Android.bp}}
```

_hello_rust/src/main.rs_:

```rust,ignore
{{#include library/src/main.rs:main}}
```

_hello_rust/src/lib.rs_:

```rust,ignore
{{#include library/src/lib.rs:greeting}}
```

You build, push, and run the binary like before:

```shell
{{#include ../build_all.sh:hello_rust_with_dep}}
```

```text
Hello Bob, it is very
nice to meet you!
```

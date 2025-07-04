# Rust Libraries

You use `rust_library` to create a new Rust library for Android.

Here we declare a dependency on two libraries:

- `libgreeting`, which we define below,
- `libtextwrap`, which is a crate already vendored in
  [`external/rust/android-crates-io/crates/`][crates].

[crates]: https://cs.android.com/android/platform/superproject/main/+/main:external/rust/android-crates-io/crates/

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

<details>

- Go through the build steps and demonstrate them running in your emulator.

- A Rust crate named `greetings` must be built by a rule called `libgreetings`.
  Note how the Rust code uses the crate name, as is normal in Rust.

- Again, the build rules enforce that we add documentation comments to all
  public items.

</details>

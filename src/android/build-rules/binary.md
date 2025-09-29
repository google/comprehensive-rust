# Rust Binaries

Let's start with a simple application. At the root of an AOSP checkout, create
the following files:

_hello_rust/Android.bp_:

```javascript
{{#include binary/Android.bp}}
```

_hello_rust/src/main.rs_:

```rust
{{#include binary/src/main.rs:main}}
```

You can now build, push, and run the binary:

```shell
{{#include ../build_all.sh:hello_rust}}
```

```text
Hello from Rust!
```

<details>

- Go through the build steps and demonstrate them running in your emulator.

- Notice the extensive documentation comments? The Android build rules enforce
  that all modules have documentation. Try removing it and see what error you
  get.

- Stress that the Rust build rules look like the other Soong rules. This is by
  design, to make using Rust as easy as C++ or Java.

</details>

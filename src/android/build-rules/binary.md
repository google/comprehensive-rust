# Rust Binaries

Let us start with a simple application. At the root of an AOSP checkout, create
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

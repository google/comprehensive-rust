# Including `unsafe` Rust Code

Unsafe Rust code is forbidden in `rust_static_library` by default --- it won't
compile. If you need unsafe Rust code, add `allow_unsafe = true` to the gn
target. (Later in the course we'll see circumstances where this is necessary.)

```gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [
    "lib.rs",
    "hippopotamus.rs"
  ]
  allow_unsafe = true
}
```

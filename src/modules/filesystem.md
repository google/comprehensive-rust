# Filesystem Hierarchy

The module content can be omitted:

```rust,editable,compile_fail
mod garden;
```

The `garden` module content is found at:

* `src/garden.rs` (modern Rust 2018 style)
* `src/garden/mod.rs` (older Rust 2015 style)

Similarly, a `garden::vegetables` module can be found at:

* `src/garden/vegetables.rs` (modern Rust 2018 style)
* `src/garden/vegetables/mod.rs` (older Rust 2015 style)

The `crate` root is in:

* `src/lib.rs` (for a library crate)
* `src/main.rs` (for a binary crate)

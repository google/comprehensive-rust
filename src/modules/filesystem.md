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

<details>

* The change from `module/mod.rs` to `module.rs` doesn't preclude the use of submodules in Rust 2018.
  (It was mandatory in Rust 2015.)

  The following is valid:

  ```ignore
  src/
  ├── main.rs
  ├── top_module.rs
  └── top_module/
      └── sub_module.rs
  ```

* The main reason for the change is to prevent many files named `mod.rs`, which can be hard
  to distinguish in IDEs.

* Rust will look for modules in `modulename/mod.rs` and `modulename.rs`, but this can be changed
  with a compiler directive:

  ```rust,ignore
  #[path = "some/path.rs"]
  mod some_module { }
  ```

  This is useful, for example, if you would like to place tests for a module in a file named
  `some_module_test.rs`, similar to the convention in Go.

</details>

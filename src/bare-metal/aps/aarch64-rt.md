# aarch64-rt

The `aarch64-rt` crate provides the assembly entry point and exception vector
that we implemented before. We just need to mark our main function with the
`entry!` macro.

It also provides the `initial_pagetable!` macro to let us define an initial
static pagetable in Rust, rather than in assembly code like we did before.

```rust,editable,compile_fail
{{#include examples/src/main_rt.rs:main}}
```

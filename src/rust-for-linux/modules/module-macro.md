---
minutes: 2
---

# The `module!` Macro

A kernel module itself is declared with the
[`module!`](https://rust.docs.kernel.org/macros/macro.module.html) macro.

Here we specify the type for the module, upon which we will implement the
`kernel::Module` trait, as well as metadata like the module's name and
description.

```rust
module! {
    type: RustMinimal,
    name: "rust_minimal",
    author: "Rust for Linux Contributors",
    description: "Rust minimal sample",
    license: "GPL",
}

struct RustMinimal;
```

# Bitflags

The [`bitflags`](https://crates.io/crates/bitflags) crate is useful for working
with bitflags.

```rust,editable,compile_fail
{{#include ../examples/src/pl011.rs:Flags}}
```

<details>

- The `bitflags!` macro creates a newtype something like `Flags(u16)`, along
  with a bunch of method implementations to get and set flags.

</details>

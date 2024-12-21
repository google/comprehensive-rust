# More traits

We derived the `Debug` trait. It would be useful to implement a few more traits
too.

```rust,editable,compile_fail
{{#include ../examples/src/pl011_minimal.rs:Traits}}
```

<details>

- Implementing `Write` lets us use the `write!` and `writeln!` macros with our
  `Uart` type.

- `Send` is an auto-trait, but not implemented automatically because it is not
  implemented for pointers.

</details>

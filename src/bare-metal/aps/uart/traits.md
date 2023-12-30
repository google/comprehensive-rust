# More traits

We derived the `Debug` trait. It would be useful to implement a few more traits
too.

```rust,editable,compile_fail
{{#include ../examples/src/pl011_minimal.rs:Traits}}
```

<details>

- Implementing `Write` lets us use the `write!` and `writeln!` macros with our
  `Uart` type.
- Run the example in QEMU with `make qemu_minimal` under
  `src/bare-metal/aps/examples`.

</details>

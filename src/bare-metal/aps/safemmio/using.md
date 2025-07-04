# Using It

Let's write a small program using our driver to write to the serial console, and
echo incoming bytes.

```rust,editable,compile_fail
{{#include ../examples/src/main_safemmio.rs:main}}
```

<details>

- Run the example in QEMU with `make qemu_safemmio` under
  `src/bare-metal/aps/examples`.

</details>

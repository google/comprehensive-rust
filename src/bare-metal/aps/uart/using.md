# Using it

Let's write a small program using our driver to write to the serial console.

```rust,editable,compile_fail
{{#include ../examples/src/main_minimal.rs:main}}
```

<details>

- As in the [inline assembly](../inline-assembly.md) example, this `main`
  function is called from our entry point code in `entry.S`. See the speaker
  notes there for details.
- Run the example in QEMU with `make qemu_minimal` under
  `src/bare-metal/aps/examples`.

</details>

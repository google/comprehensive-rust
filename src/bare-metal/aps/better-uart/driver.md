# Driver

Now let's use the new `Registers` struct in our driver.

```rust,editable,compile_fail
{{#include ../examples/src/pl011_struct.rs:Uart}}
```

<details>

- Note the use of `&raw const` / `&raw mut` to get pointers to individual fields
  without creating an intermediate reference, which would be unsound.
- The example isn't included in the slides because it is very similar to the
  `safe-mmio` example which comes next. You can run it in QEMU with `make qemu`
  under `src/bare-metal/aps/examples` if you need to.

</details>

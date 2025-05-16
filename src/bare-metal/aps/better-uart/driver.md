# Driver

Now let's use the new `Registers` struct in our driver.

```rust,editable,compile_fail
{{#include ../examples/src/pl011.rs:Uart}}
```

<details>

- `UniqueMmioPointer` is a wrapper around a raw pointer to an MMIO device or
  register. The caller of `UniqueMmioPointer::new` promises that it is valid and
  unique for the given lifetime, so it can provide safe methods to read and
  write fields.
- These MMIO accesses are generally a wrapper around `read_volatile` and
  `write_volatile`, though on aarch64 they are instead implemented in assembly
  to work around a bug where the compiler can emit instructions that prevent
  MMIO virtualisation.
- The `field!` and `field_shared!` macros internally use `&raw mut` and
  `&raw const` to get pointers to individual fields without creating an
  intermediate reference, which would be unsound.

</details>

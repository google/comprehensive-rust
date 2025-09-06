# Driver

Now let's use the new `Registers` struct in our driver.

```rust,editable,compile_fail
{{#include ../examples/src/pl011.rs:Uart}}
```

<details>

- The driver no longer needs any unsafe code!
- `UniqueMmioPointer` is a wrapper around a raw pointer to an MMIO device or
  register. The caller of `UniqueMmioPointer::new` promises that it is valid and
  unique for the given lifetime, so it can provide safe methods to read and
  write fields.
- Note that `Uart::new` is now safe; `UniqueMmioPointer::new` is unsafe instead.
- These MMIO accesses are generally a wrapper around `read_volatile` and
  `write_volatile`, though on aarch64 they are instead implemented in assembly
  to work around a bug where the compiler can emit instructions that prevent
  MMIO virtualization.
- The `field!` and `field_shared!` macros internally use `&raw mut` and
  `&raw const` to get pointers to individual fields without creating an
  intermediate reference, which would be unsound.
- `field!` needs a mutable reference to a `UniqueMmioPointer`, and returns a
  `UniqueMmioPointer` that allows reads with side effects and writes.
- `field_shared!` works with a shared reference to either a `UniqueMmioPointer`
  or a `SharedMmioPointer`. It returns a `SharedMmioPointer` that only allows
  pure reads.

</details>

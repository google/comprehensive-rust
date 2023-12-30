# Inline assembly

Sometimes we need to use assembly to do things that aren't possible with Rust
code. For example, to make an HVC (hypervisor call) to tell the firmware to
power off the system:

```rust,editable,compile_fail
{{#include examples/src/main_psci.rs:main}}
```

(If you actually want to do this, use the [`smccc`][1] crate which has wrappers
for all these functions.)

<details>

- PSCI is the Arm Power State Coordination Interface, a standard set of
  functions to manage system and CPU power states, among other things. It is
  implemented by EL3 firmware and hypervisors on many systems.
- The `0 => _` syntax means initialise the register to 0 before running the
  inline assembly code, and ignore its contents afterwards. We need to use
  `inout` rather than `in` because the call could potentially clobber the
  contents of the registers.
- This `main` function needs to be `#[no_mangle]` and `extern "C"` because it is
  called from our entry point in `entry.S`.
- `_x0`–`_x3` are the values of registers `x0`–`x3`, which are conventionally
  used by the bootloader to pass things like a pointer to the device tree.
  According to the standard aarch64 calling convention (which is what
  `extern "C"` specifies to use), registers `x0`–`x7` are used for the first 8
  arguments passed to a function, so `entry.S` doesn't need to do anything
  special except make sure it doesn't change these registers.
- Run the example in QEMU with `make qemu_psci` under
  `src/bare-metal/aps/examples`.

</details>

[1]: https://crates.io/crates/smccc

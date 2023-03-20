# Inline assembly

Sometimes we need to use assembly to do things that aren't possible with Rust code. For example,
to make an <abbr title="hypervisor call">HVC</abbr> to tell the firmware to power off the system:

```rust,editable,compile_fail
{{#include examples/src/main_psci.rs:main}}
```

(If you actually want to do this, use the [`psci`][1] crate which has wrappers for all these functions.)

<details>

* PSCI is the Arm Power State Coordination Interface, a standard set of functions to manage system
  and CPU power states, among other things. It is implemented by EL3 firmware and hypervisors on
  many systems.
* The `0 => _` syntax means initialise the register to 0 before running the inline assembly code,
  and ignore its contents afterwards. We need to use `inout` rather than `in` because the call could
  potentially clobber the contents of the registers.

</details>

[1]: https://crates.io/crates/psci

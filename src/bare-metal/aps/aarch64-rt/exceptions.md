<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exceptions

`aarch64-rt` provides a trait to define exception handlers, and a macro to
generate the assembly code for the exception vector to call them.

The trait has default implementations for each method which simply panic, so we
can omit methods for exceptions we don't expect to happen.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/exceptions_rt.rs:exceptions}}
```

<details>

- The `exception_handlers` macro generates a `global_asm!` block with the
  exception vector to call into the Rust code, similar to the `exceptions.S` we
  had before.
- `RegisterStateRef` wraps a reference to the stack frame where the register
  values were saved by the assembly code when the exception happed. This can be
  used for example to extract the parameters for an SMC or HVC call from a lower
  EL, and update the values to be restored when the exception handler returns.

</details>

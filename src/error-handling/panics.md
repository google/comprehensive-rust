---
minutes: 3
---

<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Panics

In case of a fatal runtime error, Rust triggers a "panic":

```rust,editable,should_panic
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let v = vec![10, 20, 30];
    dbg!(v[100]);
}
```

- Panics are for unrecoverable and unexpected errors.
  - Panics are symptoms of bugs in the program.
  - Runtime failures like failed bounds checks can panic.
  - Assertions (such as `assert!`) panic on failure.
  - Purpose-specific panics can use the `panic!` macro.
- A panic will "unwind" the stack, dropping values just as if the functions had
  returned.
- Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.

<details>

By default, a panic will cause the stack to unwind. The unwinding can be caught:

```rust,editable
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| "No problem here!");
    dbg!(result);

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    dbg!(result);
}
```

- Catching is unusual; do not attempt to implement exceptions with
  `catch_unwind`!
- This can be useful in servers which should keep running even if a single
  request crashes.
- This does not work if `panic = 'abort'` is set in your `Cargo.toml`.

</details>

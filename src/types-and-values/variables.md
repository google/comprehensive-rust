---
minutes: 5
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Variables

Rust provides type safety via static typing. Variable bindings are made with
`let`:

```rust,editable,warnunused
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
```

<details>

- Uncomment the `x = 20` to demonstrate that variables are immutable by default.
  Add the `mut` keyword to allow changes.

- Warnings are enabled for this slide, such as for unused variables or
  unnecessary `mut`. These are omitted in most slides to avoid distracting
  warnings. Try removing the mutation but leaving the `mut` keyword in place.

- The `i32` here is the type of the variable. This must be known at compile
  time, but type inference (covered later) allows the programmer to omit it in
  many cases.

</details>

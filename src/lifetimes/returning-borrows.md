---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Returning Borrows

But we can also have our function return a reference! This means that a borrow
flows back out of a function:

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn identity(x: &i32) -> &i32 {
    x
}

fn main() {
    let mut x = 123;

    let out = identity(&x);

    // x = 5; // 🛠️❌ `x` is still borrowed!

    dbg!(out);
}
```

<details>

- Rust functions can return references, meaning that a borrow can flow back out
  of a function.

- If a function returns a reference (or another kind of borrow), it was likely
  derived from one of its arguments. This means that the return value of the
  function will extend the borrow for one or more argument borrows.

- This case is still fairly simple, in that only one borrow is passed into the
  function, so the returned borrow has to be the same one.

</details>

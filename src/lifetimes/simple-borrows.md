---
minutes: 3
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Borrowing with Functions

As part of borrow checking, the compiler needs to reason about how borrows flow
into and out of functions. In the simplest case borrows last for the duration of
the function call:

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn borrows(x: &i32) {
    dbg!(x);
}

fn main() {
    let mut val = 123;

    // Borrow `val` for the function call.
    borrows(&val);

    // Borrow has ended and we're free to mutate.
    val += 5;
}
```

<details>

- In this example we borrow `val` for the call to `borrows`. This would limit
  our ability to mutate `val`, but once the function call returns the borrow has
  ended and we're free to mutate again.

</details>

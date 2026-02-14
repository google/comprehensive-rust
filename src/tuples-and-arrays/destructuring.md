---
minutes: 5
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Patterns and Destructuring

Rust supports using pattern matching to destructure a larger value like a tuple
into its constituent parts:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn main() {
    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) { "ordered" } else { "unordered" }
    );
}
```

<details>

- The patterns used here are "irrefutable", meaning that the compiler can
  statically verify that the value on the right of `=` has the same structure as
  the pattern.
- A variable name is an irrefutable pattern that always matches any value, hence
  why we can also use `let` to declare a single variable.
- Rust also supports using patterns in conditionals, allowing for equality
  comparison and destructuring to happen at the same time. This form of pattern
  matching will be discussed in more detail later.
- Edit the examples above to show the compiler error when the pattern doesn't
  match the value being matched on.

</details>

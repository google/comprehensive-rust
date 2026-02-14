<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `for`

The [`for` loop](https://doc.rust-lang.org/std/keyword.for.html) iterates over
ranges of values or the items in a collection:

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
}
```

<details>

- Under the hood `for` loops use a concept called "iterators" to handle
  iterating over different kinds of ranges/collections. Iterators will be
  discussed in more detail later.
- Note that the first `for` loop only iterates to `4`. Show the `1..=5` syntax
  for an inclusive range.

</details>

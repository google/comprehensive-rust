---
minutes: 5
---

<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Tuples

<!-- mdbook-xgettext: skip -->

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}
```

<details>

- Like arrays, tuples have a fixed length.

- Tuples group together values of different types into a compound type.

- Fields of a tuple can be accessed by the period and the index of the value,
  e.g. `t.0`, `t.1`.

- The empty tuple `()` is referred to as the "unit type" and signifies absence
  of a return value, akin to `void` in other languages.

- Unlike arrays, tuples cannot be used in a `for` loop. This is because a `for`
  loop requires all the elements to have the same type, which may not be the
  case for a tuple.

- There is no way to add or remove elements from a tuple. The number of elements
  and their types are fixed at compile time and cannot be changed at runtime.

</details>

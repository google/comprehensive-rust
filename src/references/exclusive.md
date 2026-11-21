---
minutes: 5
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exclusive References

Exclusive references, also known as mutable references, allow changing the value
they refer to. They have type `&mut T`.

<!-- mdbook-xgettext: skip -->

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}
```

<details>

Key points:

- "Exclusive" means only this reference can access the value: no other reference
  (shared or exclusive) may coexist. A borrow stays active until its last use,
  not until the end of the scope, so to observe the restriction, add a
  conflicting access before that last use — for example, read `&point.0` or
  write to `point.0` on the line before `*x_coord = 20;`. Adding it after the
  last use compiles, because the borrow has already ended.

- Be sure to note the difference between `let mut x_coord: &i32` and
  `let x_coord: &mut i32`. The first one is a shared reference that can be bound
  to different values, while the second is an exclusive reference to a mutable
  value.

</details>

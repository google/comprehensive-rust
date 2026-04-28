---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `as_` and `_ref`: reference conversions

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> Rc<T> {
    // Very common on container types, see how it's also on Option.
    fn as_ref(&self) -> &T;

    fn as_ptr(&self) -> *const T;
}

impl<T> Option<T> {
    fn as_ref(&self) -> Option<&T>;

    fn as_slice(&self) -> &[T];
}
```

<details>

- Method that returns a borrow of the primary piece of contained data.

- The borrowing relationship is most often straightforward: the return value is
  a reference that borrows `self`.

- The returned value could borrow `self` only logically, for example, `as_ptr()`
  methods return an unsafe pointer. The borrow checker does not track borrowing
  for pointers.

- The type implementing an "as" method should contain one primary piece of data
  that is being borrowed out.

  - The "as" naming convention does not work if the data type is an aggregate of
    many fields without an obvious primary one.

  - If you have two reference getters that you need to distinguish, use the
    `_ref` suffix.

</details>

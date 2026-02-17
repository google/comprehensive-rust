---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Aside: `into_inner`

Special case of `into`: for exclusive pointer types or newtypes, extract the
internal value.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    fn into_inner(self) -> T;
}

pub struct NonZeroU32(u32);

impl NonZeroU32 {
    fn into_inner(self) -> u32;
}

impl<T> Cell<T> {
    fn into_inner(self) -> T;
}
```

<details>

- `into_inner` is a method usually found on newtypes: types whose main purpose
  is to wrap around an existing type and be semantically distinct from other
  uses of that inner type.

This kind of method is also found on types like `Cell`, which exclusively own
the internal data.

The purpose of this kind of method is to consume the "wrapper" type and return
the "contained" value.

- When defining a type with exactly one field, consider if it makes sense to
  implement an `into_inner` method that consumes `self` and returns the field as
  an owned value.

  Don't write a method like this if more fields will be added in the future.

</details>

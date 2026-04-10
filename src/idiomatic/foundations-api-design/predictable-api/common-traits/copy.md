---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Copy

Like `Clone`, but indicates the type is can be bitwise copied.

Derivable: ✅

When to implement: If possible, but with caveats.

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// Copy is just a marker trait with Clone as a supertrait.
// pub trait Copy: Clone { }

#[derive(Debug, Clone, Copy)]
pub struct Copyable(u8, u16, u32, u64);

fn main() {
    let copyable = Copyable(1, 2, 3, 4);
    let copy = copyable; // Implicit copy operation
    dbg!(copyable);
    dbg!(copy);
}
```

<details>
- Clone represents a deep clone, and so does copy, but copy suggests to the compiler that a value can be copied bitwise.

- When not to implement/derive: If you do not want to implicitly create copies
  when dereferencing values of a type, do not implement this trait.

  Copy enables implicit duplication, so be careful about what types you're
  implementing this on.

- Ask the class: Can a type with heap data (`Vec`, `BTreeMap`, `Rc`, etc.) be
  copy? Should it be?

  It both cannot and should not, this is a misuse of this trait.

  Bitwise copying on these types would mean types with heap data would no longer
  have exclusive ownership of a pointer, breaking the invariants usually upheld
  by Rust and its ecosystem.

  Multiple `Vec`s would point to the same data in memory. Adding and removing
  data would only update individual `Vec`s length and capacity values. The same
  for `BTreeMap`.

  Bitwise copying of `Rc`s would not update the reference counting value within
  the pointers, meaning there could be two instances of a `Rc` value that
  believe themselves to be the only `Rc` for that pointer. Once one of them is
  destroyed, the reference count will become 0 on one of them and the inner
  value dropped despite there being another `Rc` still alive.

</details>

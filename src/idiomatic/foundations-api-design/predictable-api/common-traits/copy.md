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

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
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

- `Clone` represents an explicit, user-defined copy operation. `Copy` represents
  an implicit, bitwise copy.

- Should generally only be implemented on "plain data" types that should act
  like primitive values. For example, primitive numeric types for a linear
  algebra library.

- Has the same caveat as `Clone`: If duplicating the values would break an
  invariant, the type shouldn't implement `Copy`.

- Always derive `Clone` and `Copy` together! Do _not_ manually implement `Clone`
  when implementing `Copy`.

  - Copy operations do not invoke the `clone` method, so a custom `Clone` impl
    can have different behavior than an implicit copy operation. Deriving both
    `Clone` and `Copy` together ensures that calling `clone` will give the same
    result as invoking a copy.

- Cannot be implemented on types with `Drop` or non-`Copy` fields.

  - Ask the class: Why can't a type with heap data (`Vec`, `BTreeMap`, `Rc`,
    etc.) be `Copy`?

    Bitwise copying on these types would mean types with heap data would no
    longer have exclusive ownership of a pointer, breaking the invariants
    usually upheld by Rust and its ecosystem.

    Multiple `Vec`s would point to the same data in memory. Adding and removing
    data would only update individual `Vec`s length and capacity values. The
    same for `BTreeMap`.

    Bitwise copying of `Rc`s would not update the reference counting value
    within the pointers, meaning there could be two instances of a `Rc` value
    that believe themselves to be the only `Rc` for that pointer. Once one of
    them is destroyed, the reference count will become 0 on one of them and the
    inner value dropped despite there being another `Rc` still alive.

</details>

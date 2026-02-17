---
minutes: 15
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# PhantomData 3/4: Lifetimes for External Resources

The invariants of external resources often match what we can do with lifetime
rules.

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// use std::marker::PhantomData;

/// Direct FFI to a database library in C.
/// We got this API as is, we have no influence over it.
mod ffi {
    pub type DatabaseHandle = u8; // maximum 255 databases open at the same time

    fn database_open(name: *const std::os::raw::c_char) -> DatabaseHandle {
        unimplemented!()
    }
    // ... etc.
}

struct DatabaseConnection(ffi::DatabaseHandle);

struct Transaction<'a>(&'a mut DatabaseConnection);

impl DatabaseConnection {
    fn new_transaction(&mut self) -> Transaction<'_> {
        Transaction(self)
    }
}

fn main() {}
```

<details>

- Remember the transaction API from the
  [Aliasing XOR Mutability](./aliasing-xor-mutability.md) example.

  We held onto a mutable reference to the database connection within the
  transaction type to lock out the database while a transaction is active.

  In this example, we want to implement a `Transaction` API on top of an
  external, non-Rust API.

  We start by defining a `Transaction` type that holds onto
  `&mut DatabaseConnection`.

- Ask: What are the limits of this implementation? Assume the `u8` is accurate
  implementation-wise and enough information for us to use the external API.

  Expect:
  - Indirection takes up 7 bytes more than we need to on a 64-bit platform, as
    well as costing a pointer dereference at runtime.

- Problem: We want the transaction to borrow the database connection that
  created it, but we don't want the `Transaction` object to store a real
  reference.

- Ask: What happens when we remove the mutable reference in `Transaction` while
  keeping the lifetime parameter?

  Expect: Unused lifetime parameter!

- Like with the type tagging from the previous slides, we can bring in
  `PhantomData` to capture this unused lifetime parameter for us.

  The difference is that we will need to use the lifetime alongside another
  type, but that other type does not matter too much.

- Demonstrate: change `Transaction` to the following:

  ```rust,compile_fail
  # // Copyright 2025 Google LLC
  # // SPDX-License-Identifier: Apache-2.0
  #
  struct Transaction<'a> {
      connection: DatabaseConnection,
      _phantom: PhantomData<&'a mut DatabaseConnection>,
  }
  ```

  Update the `DatabaseConnection::new_transaction()` method:

  ```rust,compile_fail
  # // Copyright 2025 Google LLC
  # // SPDX-License-Identifier: Apache-2.0
  #
  impl DatabaseConnection {
      fn new_transaction<'a>(&'a mut self) -> Transaction<'a> {
          Transaction { connection: DatabaseConnection(self.0), _phantom: PhantomData }
      }
  }
  ```

  This gives an owned database connection that is tied to the
  `DatabaseConnection` that created it, but with less runtime memory footprint
  that the store-a-reference version did.

  Because `PhantomData` is a zero-sized type (like `()` or
  `struct MyZeroSizedType;`), the size of `Transaction` is now the same as `u8`.

  The implementation that held onto a reference instead was as large as a
  `usize`.

## More to Explore

- This way of encoding relationships between types and values is very powerful
  when combined with unsafe, as the ways one can manipulate lifetimes becomes
  almost arbitrary. This is also dangerous, but when combined with tools like
  external, mechanically-verified proofs we can safely encode
  cyclic/self-referential types while encoding lifetime & safety expectations in
  the relevant data types.

- The [GhostCell (2021)](https://plv.mpi-sws.org/rustbelt/ghostcell/) paper and
  its [relevant implementation](https://gitlab.mpi-sws.org/FP/ghostcell) show
  this kind of work off. While the borrow checker is restrictive, there are
  still ways to use escape hatches and then _show that the ways you used those
  escape hatches are consistent and safe._

</details>

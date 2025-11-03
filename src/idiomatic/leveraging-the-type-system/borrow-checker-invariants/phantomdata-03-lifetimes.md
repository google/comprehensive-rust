---
minutes: 15
---

# PhantomData 2/2: Tagging with Lifetimes

```rust,editable
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

  But here we need to account for the lifetime behavior of external APIs in this
  case.

- We can also save 7 bytes in the size of `Transaction` by having it be owned vs
  being a reference on a 64bit platform.

- We can use `PhantomData` to capture lifetime parameters that don't have "real"
  borrowed values present by making the type parameter of `PhantomData` use that
  lifetime.

- Demonstrate: change `Transaction` to the following

  ```rust,compile_fail
  pub struct Transaction<'a> {
      connection: DatabaseConnection,
      _phantom: PhantomData<&'a ()>,
  }
  ```

  Change the `new_transaction` function for `DatabaseConnection` to the
  following:

  ```rust,compile_fail
  fn new_transaction(&'a self) -> Transaction<'a> {
      Transaction { connection: DatabaseConnection(self.0), _phantom: PhantomData }
  }
  ```

  This gives an owned database connection to a specific database as per the FFI,
  and creates a compile-time only relationship between that connection and the
  "source" `DatabaseConnection` that created it.

- Demonstrate: We can give each `Transaction` an owned `DatabaseConnection`, but
  change the constructor to be a method of `DatabaseConnection` itself.

## More to Explore

- This way of encoding information in types is very powerful when combined with
  unsafe, as the ways one can manipulate lifetimes becomes almost arbitrary.
  This is also dangerous, but when combined with tools like external,
  mechanically-verified proofs we can safely encode cyclic/self-referential
  types while encoding lifetime & safety expectations in the relevant data
  types.

- The [GhostCell (2021)](https://plv.mpi-sws.org/rustbelt/ghostcell/) paper and
  its [relevant implementation](https://gitlab.mpi-sws.org/FP/ghostcell) show
  this kind of work off. While the borrow checker is restrictive, there are
  still ways to use escape hatches and then _show that the ways you used those
  escape hatches are consistent and safe._

</details>

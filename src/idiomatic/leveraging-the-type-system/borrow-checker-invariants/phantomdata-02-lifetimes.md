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
  transaction type, this asserts that while we have a transaction nothing else
  can use the database handle.

  But this isn't the way database connections tend to work in practice. We can
  have multiple connections.

  But we could theoretically be connected to multiple distinct databases.

  Ask: how could we associate multiple connections to the same database in the
  type system?

  Expect: Just have multiple connections without annotating where they're from
  in the type system. Or, create a new tag type for each database.

  What's wrong with type tagging? It doesn't encode a relationship between
  specific variables the way that lifetime annotations do.

- But what we want to do is use the type system as much as possible to express
  what is possible.

  What is possible is to have multiple connections to the same database,
  especially in the context of transactions being committed on the same machine,
  so we want to encode that while maintaining a _relationship_ between databases
  and transactions.

- Additionally: We can save 7 bytes in the size of `Transaction` by having it be
  owned vs being a reference on a 64bit platform.

- We can use `PhantomData` to use Lifetime parameters that don't have "real"
  values borrowed by making the type parameter of `PhantomData` use that
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
  but creates a relationship between that connection and the "source"
  `DatabaseConnection` that created it.

- Demonstrate: We can give each `Transaction` an owned `DatabaseConnection`, but
  change the constructor to instead be a method of `DatabaseConnection` itself.

- [`BorrowedFd`](https://rust-lang.github.io/rfcs/3128-io-safety.html#ownedfd-and-borrowedfdfd)
  uses these captured lifetimes to enforce the invariant that "if this file
  descriptor exists, the OS file descriptor is still open."

  `BorrowedFd`'s lifetime parameter demands that there exists another value (in
  this case a file, in the Unix sense) in your program that lasts as long as the
  `BorrowedFd` or outlives it.

  This has been encoded by the API designer to mean _that other value is what
  keeps the access to the file open_.

  Because `BorrowedFd` has a lifetime parameter from that other value, users of
  the API can assume "this file descriptor existing means the file is open, and
  we don't need to manage or check that external state itself."

  Its counterpart `OwnedFd` is instead a file descriptor that closes that file
  on drop.

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

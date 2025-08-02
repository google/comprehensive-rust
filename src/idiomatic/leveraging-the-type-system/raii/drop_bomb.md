# Drop Bombs: Enforcing API Correctness

Use `Drop` to enforce invariants and detect incorrect API usage. A "drop bomb"
panics if a value is dropped without being explicitly finalized.

This pattern is often used when the finalizing operation (like `commit()` or
`rollback()`) needs to return a `Result`, which cannot be done from `Drop`.

```rust,editable
struct Transaction {
    active: bool,
}

impl Transaction {
    /// Begin a [`Transaction`].
    ///
    /// ## Panics
    ///
    /// Panics if the transaction is dropped without
    /// calling [`Self::commit`] or [`Self::rollback`].
    fn start() -> Self {
        Self { active: true }
    }

    fn commit(mut self) -> io::Result<()> {
        writeln!(io::stdout(), "COMMIT")?;
        self.active = false;
        Ok(())
    }

    fn rollback(mut self) -> io::Result<()> {
        writeln!(io::stdout(), "ROLLBACK")?;
        self.active = false;
        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.active {
            panic!("Transaction dropped without commit or rollback!");
        }
    }
}

fn main() -> io::Result<()> {
    let tx = Transaction::start();

    if some_condition() {
        tx.commit()?;
    } else {
        tx.rollback()?;
    }

    // Uncomment to see the panic:
    // let tx2 = Transaction::start();

    Ok(())
}

fn some_condition() -> bool {
    true // change to false to test rollback
}
```

<details>

- This pattern ensures that a value like `Transaction` cannot be silently
  dropped in an unfinished state. The destructor panics if neither `commit()`
  nor `rollback()` has been called.

- A common reason to use this pattern is when cleanup cannot be done in `Drop`,
  either because it is fallible or asynchronous. For example, most databases do
  not allow rollback to be safely handled inside `drop()` alone.

- This pattern is appropriate even in public APIs. It can help users catch bugs
  early when they forget to explicitly finalize a transactional object.

- If a value can be safely cleaned up in `Drop`, consider falling back to that
  behavior in Release mode and panicking only in Debug. This decision should be
  made based on the guarantees your API provides.

- Panicking in Release builds is a valid choice if silent misuse could lead to
  serious correctness issues or security concerns.

## Additional Patterns

- [`Option<T>` with `.take()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take):
  A common pattern inside `Drop` to move out internal values and prevent double
  drops.

  ```rust,compile_fail
  impl Drop for MyResource {
      fn drop(&mut self) {
          if let Some(handle) = self.handle.take() {
              // do cleanup with handle
          }
      }
  }
  ```

- [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html):
  Prevents automatic destruction and gives full manual control. Requires
  `unsafe`, so only use when strictly necessary.

- [`drop_bomb` crate](https://docs.rs/drop_bomb/latest/drop_bomb/): A small
  utility that panics if dropped unless explicitly defused with `.defuse()`.
  Comes with a `DebugDropBomb` variant that only activates in debug builds.

- In some systems, a value must be finalized by a specific API before it is
  dropped.

  For example, an `SshConnection` might need to be deregistered from an
  `SshServer` before being dropped, or the program panics. This helps catch
  programming mistakes during development and enforces correct teardown at
  runtime.

  See a working example in the Rust playground:
  <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3223f5fa5e821cd32461c3af7162cd55>

</details>

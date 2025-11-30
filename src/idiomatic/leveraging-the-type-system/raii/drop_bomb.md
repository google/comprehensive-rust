# Drop Bombs: Enforcing API Correctness

Use `Drop` to enforce invariants and detect incorrect API usage. A "drop bomb"
panics if a value is dropped without being explicitly finalized.

This pattern is often used when the finalizing operation (like `commit()` or
`rollback()`) needs to return a `Result`, which cannot be done from `Drop`.

```rust,editable
use std::io::{self, Write};

struct Transaction {
    active: bool,
}

impl Transaction {
    fn start() -> Self {
        Self { active: true }
    }

    fn commit(mut self) -> io::Result<()> {
        writeln!(io::stdout(), "COMMIT")?;
        self.active = false;
        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.active {
            panic!("Transaction dropped without commit!");
        }
    }
}

fn main() -> io::Result<()> {
    let tx = Transaction::start();
    // Use `tx` to build the transaction, then commit it.
    // Comment out the call to `commit` to see the panic.
    tx.commit()?;
    Ok(())
}
```

<details>

- A drop bomb ensures that a value like `Transaction` cannot be silently dropped
  in an unfinished state. The destructor panics if the transaction has not been
  explicitly finalized (for example, with `commit()`).

- The finalizing operation (such as `commit()`) usually take `self` by value.
  This ensures that once the transaction is finalized, the original object can
  no longer be used.

- A common reason to use this pattern is when cleanup cannot be done in `Drop`,
  either because it is fallible or asynchronous.

- This pattern is appropriate even in public APIs. It can help users catch bugs
  early when they forget to explicitly finalize a transactional object.

- If cleanup can safely happen in `Drop`, some APIs choose to panic only in
  debug builds. Whether this is appropriate depends on the guarantees your API
  must enforce.

- Panicking in Release builds is reasonable when silent misuse would cause major
  correctness or security problems.

## More to explore

Several related patterns help enforce correct teardown or prevent accidental
drops.

- The [`drop_bomb` crate](https://docs.rs/drop_bomb/latest/drop_bomb/): A small
  utility that panics if dropped unless explicitly defused with `.defuse()`.
  Comes with a `DebugDropBomb` variant that only activates in debug builds.

- In some systems, a value must be finalized by a specific API before it is
  dropped.

  For example, an `SshConnection` might need to be deregistered from an
  `SshServer` before being dropped, or the program panics. This helps catch
  programming mistakes during development and enforces correct teardown at
  runtime.

</details>

---
minutes: 10
---

# Drop Bombs: using `std::mem::forget`

```rust,editable
use std::io::{self, Write};

struct Transaction;

impl Transaction {
    fn start() -> Self {
        Transaction
    }

    fn commit(self) -> io::Result<()> {
        writeln!(io::stdout(), "COMMIT")?;

        // Defuse the drop bomb by preventing Drop from ever running.
        std::mem::forget(self);

        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // This is the "drop bomb"
        panic!("Transaction dropped without commit!");
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

This example removes the flag from the previous slide and makes the drop method
panic unconditionally. To avoid that panic on a successful commit, the commit
method now takes ownership of the transaction and calls
[`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html), which
prevents the `Drop::drop()` method from running.

If the forgotten value owned heap allocated memory that would normally be freed
in its `drop()` implementation, one consequence is a memory leak. That is not
the case for the `Transaction` in the example above, since it does not own any
heap memory.

We can avoid needing a runtime flag by using `mem::forget()` in a tactical way.
When the transaction commits successfully, we can defuse the drop bomb by
calling `std::mem::forget` on the value, which prevents its `Drop`
implementation from running.

</details>

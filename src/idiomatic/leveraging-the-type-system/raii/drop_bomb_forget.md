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

In the previous slide we saw that calling
[`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) prevents
`Drop::drop` from ever running.

Remember that `mem::forget` leaks the value. This is safe in Rust, but the
memory will not be reclaimed.

However, this avoids needing a runtime flag: when the transaction is
successfully committed, we can _defuse_ the drop bomb — meaning we prevent
`Drop` from running — by calling `std::mem::forget` on the value instead of
letting its destructor run.

</details>

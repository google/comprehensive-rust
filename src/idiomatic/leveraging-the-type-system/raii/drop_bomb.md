# Drop Bombs: Enforcing API Correctness

Use `Drop` to enforce invariants and detect incorrect API usage.
A "drop bomb" panics if not defused.

```rust
struct Transaction {
    active: bool,
}

impl Transaction {
    fn start() -> Self {
        Self { active: true }
    }

    fn commit(mut self) {
        self.active = false;
        // Dropped after this point, no panic
    }

    fn rollback(mut self) {
        self.active = false;
        // Dropped after this point, no panic
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.active {
            panic!("Transaction dropped without commit or roll back!");
        }
    }
}
```

<details>

- The example above uses the drop bomb pattern to enforce at runtime that a transaction
  is never dropped in an unfinished state. This applies to cases such as a database
  transaction that remains active in an external system.

  In this example, the programmer must finalize the transaction explicitly,
  either by committing it or rolling it back to undo any changes.

- In the context of FFI, where cross-boundary references are involved, it is often necessary
  to ensure that manually allocated memory from the guest language is cleaned up through
  an explicit call to a safe API function.

- Similar to unsafe code, it is recommended that APIs with expectations like these
  are clearly documented under a Panic section. This helps ensure that users of the API
  are aware of the consequences of misuse.

  Ideally, drop bombs should be used only in internal APIs to catch bugs early,
  without placing implicit runtime obligations on library users.

- If there is a way to restore the system to a valid state using a fallback
  in the Drop implementation, it is advisable to restrict the use of drop bombs
  to Debug mode. In Release mode, the Drop implementation could fall back to
  safe cleanup logic while still logging the incident as an error.

- Advanced use cases might also rely on the following patterns:

  - [`Option<T>` with `.take()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take):
    This allows you to move out the resource in a controlled way, preventing
    accidental double cleanup or use-after-drop errors.

  - [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html):
    A zero-cost wrapper that disables the automatic drop behavior of a value,
    making manual cleanup required and explicit.

- The [`drop_bomb` crate](https://docs.rs/drop_bomb/latest/drop_bomb/)
  provides a way to enforce that certain values are not dropped unless explicitly defused.
  It can be added to an existing struct and exposes a `.defuse()` method to make dropping safe.
  The crate also includes a `DebugDropBomb` variant for use in debug-only builds.

## More to explore

Rust does not currently support full linear types or typestate programming
in the core language. This means the compiler cannot guarantee that a resource
was used exactly once or finalized before being dropped.

Drop bombs serve as a runtime mechanism to enforce such usage invariants manually.
This is typically done in a Drop implementation that panics if a required method,
such as `.commit()`, was not called before the value went out of scope.

There is an open RFC issue and discussion about linear types in Rust:
<https://github.com/rust-lang/rfcs/issues/814>.

</details>

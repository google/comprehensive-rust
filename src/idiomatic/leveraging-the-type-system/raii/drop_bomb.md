# Drop Bombs: Enforcing API Correctness

Use `Drop` to enforce invariants and detect incorrect API usage. A "drop bomb"
panics if not defused.

---
TODO: apply feedback

I think this slide should also mention that drop bombs are useful in cases
where the finalizing operation (e.g. commit and rollback in the slide's
example) needs to return some kind of output, which means it can't be
handled normally in Drop (realistically, commit and rollback would want to
return Results to indicate if they succeeded or not). This is one of the
limitations of Drop mentioned on the previous slide, so it'd be worth
noting that this pattern is a common way to work around that limitation.

---

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
            panic!("Transaction dropped without commit or rollback!");
        }
    }
}
```

<details>

- The example above uses the drop bomb pattern to enforce at runtime that a
  transaction is never dropped in an unfinished state. This applies to cases
  such as a database transaction that remains active in an external system.

  In this example, the programmer must finalize the transaction explicitly,
  either by committing it or rolling it back to undo any changes.

- In the context of FFI, where cross-boundary references are involved, it is
  often necessary to ensure that manually allocated memory from the foreign
  language is cleaned up through an explicit call to a safe API function.

  TODO: this is a bit far-fetched, better examples suggested by reviewers:

  ```
  Let's imagine what an implementation of an SSH server could look like. An SshServer object
  that manages all connections, and an SshConnection object that represents a single connected user.
  The connection needs to be unregistered from the server object before it can be dropped -
  or else we would be leaking resources like socket descriptors.

  Say, for efficiency the connection does not have a pointer back to the server -
  so the connection can't deregister itself within its drop.
  Therefore, SshConnection::drop should panic to catch coding bugs.
  Instead, we would provide an API like SshServer::deregister(&mut self, conn: SshConnection)
  that consumes the connection, deregisters and correctly destroys it.

  -- @randomPoison agrees, this can be dropped:

  I don't think we need to mention FFI at all here, I think the decision
  of whether or not to use a drop bomb has more to do with whether or not
  there's a reasonable way to cleanup in Drop or if the finalizing
  operation needs to return anything (see also my comment on the file).
  That situation may come up in FFI, but it's not the FFI specifically
  that makes a drop bomb necessary.
  ```

  more feedback:

  ```
  It's worth noting that a very common reason you can't drop the thing is that
  the deallocation method is async and there's no async drop.
  Database transactions are an excellent example of this, in fact --
  it's typically not possible to just rollback a transaction in drop().
  ```

- APIs that are expected to panic like this should document
  the cases when a panic will occur under a `Panics` section.

  (^ TODO: this was reworded to be more minimal. Shorter the speaker
    notes the better, to make it easier to skim through as instructor)
    Original:
    > Similar to unsafe code, it is recommended that APIs with
    > expectations like
    > these are clearly documented under a Panic section.
    > This helps ensure that
    > users of the API are aware of the consequences of misuse.

  TODO: apply feedback:

  ```
  Either edit the example directly (for it to appear with the comment from the beginning),
  or add a suggested comment here for the instructor to type or paste.
  ```

  TODO: Also more feedback:

  ```
  Why should this only be used in internal APIs? For example, a library
  providing a transactional API (like in the example on this slide) would
  want to use this pattern in its public API to help users catch cases
  where they forget to finish the transaction.
  ```

- If there is a way to restore the system to a valid state using a fallback in
  the Drop implementation, it is advisable to restrict the use of drop bombs to
  Debug mode. In Release mode, the Drop implementation could fall back to safe
  cleanup logic while still logging the incident as an error.

- Advanced use cases might also rely on the following patterns:

  - [`Option<T>` with `.take()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take):
    This allows you to move out the resource in a controlled way, preventing
    accidental double cleanup or use-after-drop errors.

    TODO: apply feedback

    ```
    I think we should provide an example of this.
    I find that it's a common pattern when doing complex logic in Drop.
    This might even be worth pulling out into its own slide.
    ```

  - [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html):
    A zero-cost wrapper that disables the automatic drop behavior of a value,
    making manual cleanup required and explicit.
    This requires unsafe code to use,
    though, so it's recommended to only use this if strictly necessary.

- The [`drop_bomb` crate](https://docs.rs/drop_bomb/latest/drop_bomb/) provides
  a way to enforce that certain values are not dropped unless explicitly
  defused. It can be added to an existing struct and exposes a `.defuse()`
  method to make dropping safe. The crate also includes a `DebugDropBomb`
  variant for use in debug-only builds.

  TODO: apply feedback:

  ```
  I don't love the wording "it is advisable to restrict the use of drop
  bombs to Debug mode". I think the decision of whether to panic in
  release builds is heavily dependent on the specifics of the API in
  question, and panicking in release builds is an entirely valid decision
  imo.
  ```

</details>

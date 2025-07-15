# Scope Guards

A scope guard makes use of the `Drop` trait to run a given closure when it goes
out of scope.

```rust,editable,compile_fail
use scopeguard::{ScopeGuard, guard};
use std::{
    fs::{self, File},
    io::Write,
};

fn conditional_success() -> bool {
    true
}

fn main() {
    let path = "temp.txt";
    let mut file = File::create(path).expect("cannot create file");

    // Write something to the file
    writeln!(file, "temporary data").unwrap();

    // Create a scope guard to clean up the file unless we defuse it
    let cleanup = guard(path, |path| {
        // Errors must be handled inside the guard,
        // but cannot be propagated.
        let _ = fs::remove_file(path);
    });

    if conditional_success() {
        // Success path: we want to keep the file
        let path = ScopeGuard::into_inner(cleanup);
    } else {
        // Otherwise, the guard remains active and deletes the file on scope exit
    }
}
```

<details>

- This example demonstrates the use of
  [the `scopeguard` crate](https://docs.rs/scopeguard/latest/scopeguard/), which
  is commonly used in internal APIs to ensure that a closure runs when a scope
  exits.

  - If the cleanup logic in the example above were unconditional, the code could
    be simplified using
    [scopeguard's `defer!` macro](https://docs.rs/scopeguard/latest/scopeguard/#defer):

    ```rust,editable,compile_fail
    let path = "temp.txt";

    scopeguard::defer! {
         let _ = std::fs::remove_file(path);
    }
    ```

- If desired, the "scope guard" pattern can be implemented manually, starting as
  follows:

  ```rust
  struct ScopeGuard<T, F: FnOnce()> {
      value: Option<T>,
      drop_fn: Option<F>,
  }

  impl<T, F: FnOnce()> ScopeGuard<T, F> {
      fn guard(value: T, drop_fn: F) -> Self {
          Self { value: Some(value), drop_fn: Some(drop_fn) }
      }

      fn into_inner(mut self) -> T {
          // The drop function is discarded and will not run
          self.value.take().unwrap()
      }
  }

  impl<T, F: FnOnce()> Drop for ScopeGuard<T, F> {
      fn drop(&mut self) {
          // Run the drop function when the guard goes out of scope.
          // Note: if `into_inner` was called earlier, the drop function won't run.
          if let Some(f) = self.drop_fn.take() {
              f();
          }
      }
  }

  impl<T, F: FnOnce()> std::ops::Deref for ScopeGuard<T, F> {
      type Target = T;

      fn deref(&self) -> &T {
          // Provide shared access to the underlying value
          self.value.as_ref().unwrap()
      }
  }

  impl<T, F: FnOnce()> std::ops::DerefMut for ScopeGuard<T, F> {
      fn deref_mut(&mut self) -> &mut T {
          // Provide exclusive access to the underlying value
          self.value.as_mut().unwrap()
      }
  }
  ```

  - The `ScopeGuard` type in the `scopeguard` crate also includes a `Debug`
    implementation and a third parameter: a
    [`Strategy`](https://docs.rs/scopeguard/latest/scopeguard/trait.Strategy.html)
    that determines when the `drop_fn` should run.

    - By default, the strategy runs the drop function unconditionally. However,
      the crate also provides built-in strategies to run the drop function only
      during unwinding (due to a panic), or only on successful scope exit.

      You can also implement your own `Strategy` trait to define custom
      conditions for when the cleanup should occur.

    - Remark also that the crates' `ScopeGuard` makes use of
      [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)
      instead of `Option` to avoid automatic or premature dropping of values,
      giving precise manual control and preventing double-drops. This avoids the
      runtime overhead and semantic ambiguity that comes with using Option.

- Recalling the transaction example from
  [the drop bombs chapter](./drop_bomb.md), we can now combine both concepts:
  define a fallback that runs unless we explicitly abort early. In the success
  path, we call `ScopeGuard::into_inner` to prevent the rollback, as the
  transaction has already been committed.

  While we still cannot propagate errors from fallible operations inside the
  drop logic, this pattern at least allows us to orchestrate fallbacks
  explicitly and with whatever guarantees or limits we require.

</details>

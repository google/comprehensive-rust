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

    /*
     * TODO: Apply Feedbak:
     *
     * Consider making the example a bit more realistic: set it up
     * as if we are downloading a file through HTTP, and
     * if the connection gets interrupted, we don't want to
     * leave behind an incomplete file. I'm not asking to
     * add/change code, only to update comments,
     * and names of functions and variables.
     */

    // Create a scope guard to clean up the file unless we defuse it
    let cleanup = guard(path, |path| {
        println!("Operation failed, deleting file: {:?}", path);
        // Errors must be handled inside the guard,
        // but cannot be propagated.
        let _ = fs::remove_file(path);
    });
    /*
     * TODO: apply feedback
     *
     * I think this should be put right after let mut file, in order to:
     *
     * + emphasize that file and cleanup go together,
     *
     * + to ensure that file gets deleted even if we
     *   have a problem in writeln!().unwrap().
     *
     */

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

  TODO apply feedback:

  ```
  For large chunks of code in the speaker notes like this, it'd be nicer
  to put this in the playground and include the playground link in the
  speaker notes instead. That makes it easier to pull up the example on
  screen, instead of having to copy-paste it into a playground window
  manually.

  That said, I'm not sure this is the example we want to use. This
  re-implements the scopeguard crate's functionality, i.e. it's a general
  purpose scope guard that calls a closure. If you're implementing a scope
  guard manually, you're more likely to be implementing a guard that does
  something specific since scopeguard already exists for doing general
  purpose scope guards.

  Maybe rework this example to use the "delete a file on failure" example
  that's in the slide? I can slap together a sketch in the playground if
  it's not clear what I'm suggesting.
  ```
  - `scopeguard` also supports selecting a
    [`Strategy`](https://docs.rs/scopeguard/latest/scopeguard/trait.Strategy.html)
    to determine when the cleanup logic should run, i.e. always, only on
    successful exit, or only on unwind. The crate also supports defining custom
    strategies.
  ```
  - `scopeguard` also supports selecting a
    [`Strategy`](https://docs.rs/scopeguard/latest/scopeguard/trait.Strategy.html)
    to determine when the cleanup logic should run, i.e. always,
    only on successful exit, or only on unwind. The crate
    also supports defining custom strategies.
  ```

- Recalling the transaction example from
  [the drop bombs chapter](./drop_bomb.md), we can now combine both concepts:
  define a fallback that runs unless we explicitly abort early. In the success
  path, we call `ScopeGuard::into_inner` to prevent the rollback, as the
  transaction has already been committed.

  While we still cannot propagate errors from fallible operations inside the
  drop logic, this pattern at least allows us to orchestrate fallbacks
  explicitly and with whatever guarantees or limits we require.

  TODO: apply feedback for the above paragraph:

  ```
  Maybe move this point to the top, since it most directly explains what
  this example is doing and why. You may want to reword/reorganize this to
  merge it with the current first bullet point.
  ```

</details>

---
minutes: 30
---

# RAII and `Drop` in Practice

RAII (*Resource Acquisition Is Initialization*)
means tying the lifetime of a resource to the lifetime of a value.

Rust applies RAII automatically for memory management.
The `Drop` trait lets you extend this pattern to anything else.

```rust
use std::sync::Mutex;

fn main() {
    let mux = Mutex::new(vec![1, 2, 3]);

    {
        let mut data = mux.lock().unwrap();
        data.push(4); // lock held here
    } // lock automatically released here
}
```

<details>

- In the above example
  [the `Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
  owns its data: you can’t access the value inside without first acquiring the lock.

  `mux.lock()` returns a
  [`MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html),
  which [dereferences](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
  to the data and implements [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).

- You may recall from [the Memory Management chapter](../../memory-management/drop.md)
  that the [`Drop` trait](https://doc.rust-lang.org/std/ops/trait.Drop.html)
  lets you define what should happen when a resource is dropped.

  - In [the Blocks and Scopes chapter](../../control-flow-basics/blocks-and-scopes.md),
    we saw the most common situation where a resource is dropped:
    when the scope of its _owner_ ends at the boundary of a block (`{}`).

  - The use of
    [`std::mem::drop(val)`](https://doc.rust-lang.org/std/mem/fn.drop.html)
    allows you to _move_ a value out of scope before the block ends.

  - There are also other scenarios where this can happen,
    such as when the value owning the resource is "shadowed" by another value:

    ```rust
    let a = String::from("foo");
    let a = 3; // ^ The previous string is dropped here
               //   because we shadow its binding with a new value.
    ```

  - Recall also from [the Drop chapter](../../memory-management/drop.md)
    that for a composite type such as a `struct`, all its fields will be dropped
    when the struct itself is dropped.
    If a field implements the `Drop` trait, its `Drop::drop`
    _trait_ method will also be invoked.

- In any scenario where the stack unwinds the value, it is guaranteed
  that the [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)
  method of a value `a` will be called.

  - This holds true for happy paths such as:

    - Exiting a block or function scope.

    - Returning early with an explicit `return` statement,
      or implicitly by using
      [the Try operator (`?`)](../../error-handling/try.md)
      to early-return `Option` or `Result` values.

  - It also holds for unexpected scenarios where a `panic` is triggered, if:

    - The stack unwinds on panic (which is the default),
      allowing for graceful cleanup of resources.

      This unwind behavior can be overridden to instead
      [abort on panic](https://github.com/rust-lang/rust/blob/master/library/panic_abort/src/lib.rs).

    - No panic occurs within any of the `drop` methods
      invoked before reaching the `drop` call of the object `a`.

  - Note that
    [an explicit exit of the program](https://doc.rust-lang.org/std/process/fn.exit.html),
    as sometimes used in CLI tools, terminates the process immediately.
    In other words, the stack is not unwound in this case,
    and the `drop` method will not be called.

- `Drop` is a great fit for use cases like `Mutex`.

  When the guard goes out of scope, [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)
  is called and unlocks the mutex automatically.

  In contrast to C++ or Java, where you often have to unlock manually
  or use a `lock/unlock` pattern, Rust ensures the
  lock *cannot* be forgotten, thanks to the compiler.

- In other scenarios, the `Drop` trait shows its limitations.
  Next, we'll look at what those are and how we can
  address them.

## More to explore

To learn more about building synchronization primitives,
consider reading [*Rust Atomics and Locks* by Mara Bos](https://marabos.nl/atomics/).

The book demonstrates, among other topics, how `Drop`
and RAII work together in constructs like `Mutex`.


</details>

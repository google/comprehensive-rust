---
minutes: 30
---

# RAII and `Drop` in Practice

RAII (_Resource Acquisition Is Initialization_) means tying the lifetime of a
resource to the lifetime of a value.

Rust uses RAII for managing heap memory. The `Drop` trait lets you
extend this pattern to anything else.

```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(vec![1, 2, 3]);

    {
        let mut data = mutex.lock().unwrap();
        data.push(4); // lock held here
    } // lock automatically released here
}
```

<details>

- In the above example
  [the `Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) owns its
  data: you can’t access the value inside without first acquiring the lock.

  `mux.lock()` returns a
  [`MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html),
  which [dereferences](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) to
  the data and implements
  [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)

  TODO: consider devoting 1-2 slides to demonstrate the relevant snippets of
  Mutex and MutexGuard API

- You may recall from
  [the Memory Management segment](../../memory-management/drop.md) that the
  [`Drop` trait](https://doc.rust-lang.org/std/ops/trait.Drop.html) lets you
  define what should happen when a resource is dropped.

  - In
    [the Blocks and Scopes chapter](../../control-flow-basics/blocks-and-scopes.md),
    we saw the most common situation where a resource is dropped: when the scope
    of its _owner_ ends at the boundary of a block (`{}`).

  - The use of
    [`std::mem::drop(val)`](https://doc.rust-lang.org/std/mem/fn.drop.html)
    allows you to _move_ a value out of scope before the block ends.

  - There are also other scenarios where this can happen, such as when the value
    owning the resource is "shadowed" by another value:

    ```rust
    let a = String::from("foo");
    let a = 3; // ^ The previous string is dropped here
               //   because we shadow its binding with a new value.
    ```

  - Recall also from [the Drop chapter](../../memory-management/drop.md) that
    for a composite type such as a `struct`, all its fields will be dropped when
    the struct itself is dropped. If a field implements the `Drop` trait, its
    `Drop::drop` _trait_ method will also be invoked.

- In any scenario where the stack unwinds the value, it is guaranteed that the
  [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)
  method of a value `a` will be called.

  - This holds true for happy paths such as:

    - Exiting a block or function scope.

    - Returning early with an explicit `return` statement, or implicitly by
      using [the Try operator (`?`)](../../error-handling/try.md) to
      early-return `Option` or `Result` values.

  - It also holds for unexpected scenarios where a `panic` is triggered, if:

    - The stack is unwound on panic (which is the default), allowing for graceful
      cleanup of resources.

      (TODO: we might want to refactor this to make clear
        this also happens in normal function returns)

      This unwind behavior can be overridden to instead
      [abort on panic](https://github.com/rust-lang/rust/blob/master/library/panic_abort/src/lib.rs),
      in which case no destructors will run.

    - No panic occurs within any of the `drop` methods invoked before reaching
      the `drop` call of the object `a`.

  - Note that
    [an explicit exit of the program](https://doc.rust-lang.org/std/process/fn.exit.html),
    as sometimes used in CLI tools, terminates the process immediately. In other
    words, the stack is not unwound in this case, and the `drop` method will not
    be called.

    TODO: apply feedback:

    ```
    I think this whole point can be pulled out into its own slide.
    Talking about when Drop runs and when it doesn't is worth covering
    directly. I think you'd also want to talk about forget on that slide,
    and maybe briefly note that leaking destructors is not unsafe
    (unless you plan to cover them elsewhere).
    ```

- `Drop` is a great fit for use cases like `Mutex`.

  When the guard goes out of scope,
  [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)
  is called and unlocks the mutex automatically.

  In contrast to C++ or Java, where you often have to unlock manually or use a
  `lock/unlock` pattern, Rust ensures the lock _cannot_ be forgotten, thanks to
  the compiler.

  TODO: revisit references to C++ and Java, be careful in wording.
  E.g. C++ and Java their mutexes are also RAII based
  ([std::lock_guard](https://en.cppreference.com/w/cpp/thread/lock_guard.html), [absl::MutexLock](https://github.com/abseil/abseil-cpp/blob/master/absl/synchronization/mutex.h#L583), `synchronized(obj) {}` in Java).

  TODO: incorporate @gribozavr's feedback here:

  ```
  It can't be forgotten, but the MutexGuard can be forgot()'en intentionally, or leaked - like any other value.

  It is a good tie-in to discuss use cases for drop: it is good for cleaning up things within the scope of a process, but not the right tool for guaranteeing that something happens outside of the process (e.g., on local disk, or in another service in a distributed system).

  For example, it is a bad idea to rely exclusively on drop to clean up temporary files: if the program terminates in a way that skips running drop, temporary files will persist, and eventually the computer will run out of space. This can happen if the program crashes or leaks the value whose drop is responsible for deleting the file. In addition to a drop implementation within the program, one also needs a classic unix-style temp file reaper that runs as a separate process.
  ```

- In other scenarios, the `Drop` trait shows its limitations. Next, we'll look
  at what those are and how we can address them.

TODO: apply feedback from @gribozavr when refactoring the RAII content:

```
First, a custom File type that wraps a file descriptor. A file descriptor is a classic OS-level resource. We could show how to implement a simple read-only file type a with a minimal API: open() and read() to read a single byte. Then show how to implement Drop. Discuss when the drop() function runs, and how it isn't run when values are moved (contrast with C++ where the destructor always runs at the end of the scope, even for moved-from values). Show the forget() function, discuss its signature and what it means.

In other words, use this simple File type as an opportunity to do a 5-minute refresher on drop and move semantics. I see you're already doing it with instructor notes like "for a composite type such as a struct, all its fields will be dropped" and by mentioning the std::mem::drop() function. Let's lean more into it and make sure that during this discussion we have an example of a drop implementation on the screen.

Then we move on to Mutex. There we would focus on explaining the idea that for a mutex the "resource" is more abstract. In case of a mutex, the resource is exclusive access to the wrapped value. Thus, we need a second type - a MutexGuard - to represent that.

The mutex example is perfect to facilitate the drop x panic discussion. Maybe draft an extra slide that shows what happens by default with a naive drop implementation (the drop simply runs, no special code is needed for that), and then discuss why panics poison the mutex in Rust (there is a good chance that the code was mutating the shared data, so its invariants might be broken).
```

Also apply feedback from @djimitche:

```


It's probably only necessary to include one "callback" to Fundamentals --
the important point is to that this slide is a quick review of previous
content, and if students need a deeper refresher they can find that
content in the Fundamentals course.

That said, these speaker notes are pretty long! Is it possible to trim
this down to just call out the bits necessary for the RAII patterns
introduced here, leaving the rest to the students' memory of Fundamentals?
```

</details>

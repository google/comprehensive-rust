# The limitations of `Drop`

While `Drop` works well for cases like synchronization primitives, its use
becomes more questionable when dealing with I/O or unsafe resources.

```rust
use std::fs::File;
use std::io::{self, Write};

fn write_log() -> io::Result<()> {
    let mut file = File::create("log.txt")?;
    // ^ ownership of the (OS) file handle starts here

    writeln!(file, "Logging a message...")?;
    Ok(())
} // file handle goes out of scope here
```

<details>

- In the earlier example, our `File` resource owns a file handle provided by the
  operating system. (TODO: be careful in wording: earlier is ambiguous here.
  Better use "above".)

  [As stated in the documentation](https://doc.rust-lang.org/std/fs/struct.File.html):

  > Files are automatically closed when they go out of scope. Errors detected on
  > closing are ignored by the implementation of Drop.

- This highlights a key limitation of the `Drop` trait: it cannot propagate
  errors to the caller. In other words, fallible cleanup logic cannot be handled
  by the code using the `File`.

  This becomes clear when looking at the
  [definition of the `Drop` trait](https://doc.rust-lang.org/std/ops/trait.Drop.html):

  ```rust
  trait Drop {
      fn drop(&mut self);
  }
  ```

  Since `drop` does not return a `Result`, any error that occurs during cleanup
  cannot be surfaced or recovered from. This is by design: `drop` is invoked
  automatically when a value is popped off the stack during unwinding, leaving
  no opportunity for error handling.

  TODO: apply feedback:

  ```
  This last sentence suggests that there was no other design choice because of unwinding.
  That's not true: in C++, for example, one can throw an exception from a destructor while unwinding
  because of another exception. Throwing from a destructor is messy and error-prone
  (and pretty much every style guide tells you not to do it),
  however that is an existence proof that Rust's design choice here was not entirely forced.
  It is a good pragmatic choice for sure, but not the only one possible.

  I'd suggest to rewrite this sentence in a way that talks about infallibility
  of drop as a pragmatic design choice to keep the complexity of error handling under control
  (not as the only possible choice).
  ```

- One workaround is to panic inside `drop` when a failure occurs. However, this
  is risky—if a panic happens while the stack is already unwinding, the program
  will abort immediately, and remaining resources will not be cleaned up. (TODO:
  be careful in wording and context. E.g. here it is about external resources)

  While panicking in `drop` can serve certain purposes (see
  [the next chapter on "drop bombs"](./drop_bomb.md)), it should be used
  sparingly and with full awareness of the consequences.

- Another drawback of `drop` is that its execution is implicit and
  non-deterministic in terms of timing. You cannot control _when_ a value is
  dropped. And in fact as discussed in previous slide it might never even run at
  all, leaving the external resource in an undefined state.

  (TODO: non-deterministic is incorrect here, fix wording and description)

  (TODO: be careful with wording 'you cannot control'. As you can control, by
  impl drop)

  This matters particularly for I/O: normally you might set a timeout on
  blocking operations, but when I/O occurs in a `drop` implementation, you have
  no way to enforce such constraints.

  Returning to the `File` example: if the file handle hangs during close (e.g.,
  due to OS-level buffering or locking), the drop operation could block
  indefinitely. Since the call to `drop` happens implicitly and outside your
  control, there's no way to apply a timeout or fallback mechanism.

  TODO: apply feedback

  ```
  I see what you mean. I'd suggest to first say that drop is special because it terminates
  the object lifetime, so it is inherently a "one-shot" API.
  That has consequences: things like caller-driven timeouts or retries
  simply don't make sense - there's no object anymore after the first
  call. (Emphasizing caller-driven is important)

  This equally applies to all APIs that consume the object by value.

  The fact that drop is usually called implicitly though is not important
  here. For one, we can call it explicitly (std::mem::drop()); but if that
  wasn't available, we could have wrapped the object with drop in an
  Option, and then trigger drop by assigning None.
  ```

- For smart pointers and synchronization primitives, none of these drawbacks
  matter, since the operations are nearly instant and a program panic does not
  cause undefined behavior. The poisoned state disappears along with the
  termination of the program.

  (TODO: apply feedback: Note that the chapter does not discuss poisoned mutexes
  at the moment (I'm requesting that to be added in my comments above)

- For use cases such as I/O or FFI, it may be preferable to let the user clean
  up resources explicitly using a close function.

  However, this approach cannot be enforced at the type level. If explicit
  cleanup is part of your API contract, you might choose to panic in drop when
  the resource has not been properly closed. This can help catch contract
  violations at runtime.

  This is one situation where drop bombs are useful, which we will discuss next.

</details>

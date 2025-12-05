---
minutes: 15
---

# Drop can be skipped

There are cases where destructors will not run.

```rust,editable
#[derive(Debug)]
struct OwnedFd(i32);

impl Drop for OwnedFd {
    fn drop(&mut self) {
        println!("OwnedFd::drop() called, with raw fd: {:?}", self.0);
    }
}

impl Drop for TmpFile {
    fn drop(&mut self) {
        println!("TmpFile::drop() called with owned fd: {:?}", self.0);
        // libc::unlink("/tmp/file")
        // panic!("TmpFile::drop() panics");
    }
}

#[derive(Debug)]
struct TmpFile(OwnedFd);

impl TmpFile {
    fn open() -> Self {
        Self(OwnedFd(2))
    }

    fn close(&self) {
        panic!("TmpFile::close(): not implemented yet");
    }
}

fn main() {
    let owned_fd = OwnedFd(1);

    let file = TmpFile::open();

    std::process::exit(0);

    // std::mem::forget(file);

    // file.close();

    let _ = owned_fd;
}
```

<details>

- In the version that calls
  [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html),
  `TmpFile::drop()` is never run because `exit()` terminates the process
  immediately without any opportunity for a `drop()` method to be called.

  - You can prevent accidental use of `exit` by denying the
    [`clippy::exit`](https://rust-lang.github.io/rust-clippy/stable/index.html#exit)
    lint.

- If you remove the `std::process::exit(0)` line, each `drop()` method in this
  simple case will run in turn.

- Try uncommenting the
  [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) call.
  What do you think will happen?

  `mem::forget()` takes ownership and "forgets" about the value `file` without
  running its **destructor** `Drop::drop()`. The destructor of `owned_fd` is
  still run.

- Remove the `mem::forget()` call, then uncomment the `file.close()` call below
  it. What do you expect now?

  With the default `panic = "unwind"` setting, the stack still unwinds and
  destructors run, even when the panic starts in `main`.

  - With
    [`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic)
    no destructors are run.

- As a last step, uncomment the `panic!` inside `TmpFile::drop()` and run it.
  Ask the class: which destructors run before the abort?

  After a double panic, Rust no longer guarantees that remaining destructors
  will run:

  - Some cleanup that was already in progress may still complete (for example,
    field destructors of the value currently being dropped),
  - but anything scheduled later in the unwind path might be skipped entirely.
  - This is why we say you cannot rely solely on `drop()` for critical external
    cleanup, nor assume that a double panic aborts without running any further
    destructors.

- Some languages forbid or restrict exceptions in destructors. Rust allows
  panicking in `Drop::drop`, but it is almost never a good idea, since it can
  disrupt unwinding and lead to unpredictable cleanup. It is best avoided unless
  there is a very specific need, such as in the case of a **drop bomb**.

- A final piece of advice for this slide: do not rely **solely** on `drop()` for
  cleaning up resources that must be released even if the program crashes or a
  value is leaked. For example, deleting a temporary file in `drop()` is fine in
  a toy example, but in a real `TmpFile` implementation you would still need an
  external cleanup mechanism such as a temp file reaper.

  By contrast, some actions like unlocking a mutex are safe to rely on `drop()`
  for, since they have no lasting effects outside the process.

</details>

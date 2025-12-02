# Drop can be skipped

There are cases where destructors will not run.

```rust,editable
#[derive(Debug)]
struct OwnedFd(i32);

impl Drop for OwnedFd {
    fn drop(&mut self) {
        println!("OwnedFd::drop() called");

        println!("OwnedFd::drop() finished, with raw fd: {:?}", self.0);
    }
}

impl Drop for TmpFile {
    fn drop(&mut self) {
        println!("File::drop() called with owned fd: {:?}", self.0);

        // libc::unlink("/tmp/file")

        // panic!("File::drop() panics");
    }
}

#[derive(Debug)]
struct TmpFile(OwnedFd);

impl TmpFile {
    fn open() -> Self {
        Self(OwnedFd(3))
    }
}

fn main() {
    let file = TmpFile::open();

    std::process::exit(0);

    // std::mem::forget(file);

    // panic!("main() panics with file: {file:?}");
}
```

<details>

- In the version that calls
  [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html),
  `Foo::drop` is never run because `exit` terminates the process immediately
  without unwinding.

  - You can prevent accidental use of `exit` by denying the
    [`clippy::exit`](https://rust-lang.github.io/rust-clippy/stable/index.html#exit)
    lint.

- If you remove the `std::process::exit(0)` line, each `drop()` method will run
  in turn.

- Try uncommenting the
  [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) call.
  What do you think will happen?

  `mem::forget()` takes ownership and "forgets" about the value without running
  its **destructor** `Drop::drop()`.

- Remove the `mem::forget()` call, then uncomment the `panic!` below it. What do
  you expect now?

  With the default `panic = "unwind"` setting, the stack still unwinds and
  destructors run, even when the panic starts in `main`.

  - With
    [`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic),
    no unwinding takes place.

- Note that it is a bad idea to rely exclusively on drop to clean up temporary
  files.

  If the program terminates in a way that skips running drop, temporary files
  will persist, and eventually the computer will run out of space. This can
  happen if the program crashes or leaks the value whose drop is responsible for
  deleting the file. In addition to a drop implementation within the program,
  one also needs a classic unix-style temp file reaper that runs as a separate
  process.

- Finally, uncomment the `panic!` inside `File::drop()` and run it. Ask the
  class: which destructors run before the abort?

</details>

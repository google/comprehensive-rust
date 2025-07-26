---
minutes: 30
---

# RAII: `Drop` trait

RAII (Resource Acquisition Is Initialization) means tying the lifetime of a
resource to the lifetime of a value.

[Rust uses RAII to manage memory](https://doc.rust-lang.org/rust-by-example/scope/raii.html),
and the `Drop` trait allows you to extend this to other resources, such as file
descriptors or locks.

```rust,editable
struct FileLock;
struct File {
    stub: Option<u8>,
    lock: FileLock,
}
#[derive(Debug)]
struct Error;

impl File {
    fn open(path: &str) -> Result<Self, Error> {
        println!("acquire file descriptor: {path}");
        Ok(Self { stub: Some(1), lock: FileLock })
    }

    fn read(&mut self) -> Result<u8, Error> {
        self.stub.take().ok_or(Error)
    }

    fn close(self) -> Result<(), Error> {
        self.lock.release()
    }
}

impl FileLock {
    fn release(self) -> Result<(), Error> {
        println!("release file descriptor");
        Ok(())
    }
}

fn main() {
    let mut file = File::open("example.txt").unwrap();

    let mut content = Vec::new();
    while let Ok(byte) = file.read() {
        content.push(byte);
    }

    println!("content: {content:?}");
}
```

<details>

- The example shows how easy it is to forget releasing a file descriptor when
  managing it manually. In fact, the current code does not release it at all.
  Did anyone notice that `file.close()` is missing?

- Try inserting `file.close().unwrap();` at the end of `main`. Then try moving
  it before the loop — Rust will prevent that. Once `file` is moved, it can no
  longer be used. The borrow checker ensures we cannot access a `File` after it
  has been closed.

- Instead of relying on the programmer to remember to call `close()`, we can
  implement the `Drop` trait to handle cleanup automatically. This ties the
  resource to the lifetime of the `File` value. But note: `Drop` cannot return
  errors. Anything fallible must be handled inside the `drop()` method or
  avoided altogether.

  ```rust,compile_fail
  impl Drop for FileLock {
      fn drop(&mut self) {
          println!("release file descriptor automatically");
      }
  }
  ```

- If we keep both `drop()` and `close()`, the file descriptor is released twice.
  To avoid this, remove `close()` and rely on `Drop` alone.

- Demonstrate ownership transfer by moving the file into a separate `read_all()`
  function. The file is dropped when that local variable goes out of scope — not
  in `main`. This contrasts with C++, where the original scope always runs the
  destructor, even after moves.

- Add `panic!("oops")` at the start of `read_all()` to illustrate that `drop()`
  still runs during unwinding. Rust guarantees that destructors run during a
  panic unless the panic strategy is set to abort.

- There are exceptions where destructors will not run:
  - If a destructor panics during unwinding, the program aborts immediately.
  - The program also aborts immediately when using `std::process::exit()` or
    when the panic strategy is set to `abort`.

### More to Explore

The `Drop` trait has another important limitation: it is not `async`.

This means you cannot `await` inside a destructor, which is often needed when
cleaning up asynchronous resources like sockets, database connections, or tasks
that must notify another system before shutdown.

- Learn more:
  <https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_drop.html>
- Available on nightly:
  <https://doc.rust-lang.org/nightly/std/future/trait.AsyncDrop.html>

</details>

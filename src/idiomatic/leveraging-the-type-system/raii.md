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
pub struct File {
    stub: Option<u8>,
    lock: FileLock,
}
#[derive(Debug)]
pub struct Error;

impl File {
    pub fn open(path: &str) -> Result<Self, Error> {
        println!("acquire file descriptor: {path}");
        Ok(Self { stub: Some(1), lock: FileLock })
    }

    pub fn read(&mut self) -> Result<u8, Error> {
        self.stub.take().ok_or(Error)
    }

    pub fn close(self) -> Result<(), Error> {
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

- This example shows how easy it is to forget releasing a file descriptor when
  managing it manually. In fact, the current code does not release it at all.
  Did anyone notice that `file.close()` is missing?

- Try inserting `file.close().unwrap();` at the end of `main`. Then try moving
  it before the loop. Rust will reject this: once `file` is moved, it can no
  longer be accessed. The borrow checker enforces this statically.

- Instead of relying on the user to remember to call `close()`, we can implement
  the `Drop` trait to release the resource automatically. This ties cleanup to
  the lifetime of the `File` value. Note that `Drop` cannot return errors, so
  any fallible logic must be handled internally or avoided.

  ```rust,compile_fail
  impl Drop for FileLock {
      fn drop(&mut self) {
          println!("release file descriptor automatically");
      }
  }
  ```

- If both `drop()` and `close()` are present, the file descriptor is released
  twice. To avoid this, remove `close()` and rely solely on `Drop`.

  This also illustrates that when a parent type is dropped, the `drop()` method
  of its fields (such as `FileLock`) is automatically called — no extra code is
  needed.

- Demonstrate ownership transfer by moving the file into a `read_all()`
  function. The file is dropped when the local variable inside that function
  goes out of scope, not in `main`.

  This differs from C++, where destructors are tied to the original scope, even
  for moved-from values.

  The same mechanism underlies `std::mem::drop`, which lets you drop a value
  early:

  ```rust
  pub fn drop<T>(_x: T) {}
  ```

- Insert `panic!("oops")` at the start of `read_all()` to show that `drop()` is
  still called during unwinding. Rust ensures this unless the panic strategy is
  set to `abort`.

- There are exceptions where destructors will not run:
  - If a destructor panics during unwinding, the program aborts immediately.
  - The program also aborts when using `std::process::exit()` or when compiled
    with the `abort` panic strategy.

### More to Explore

The `Drop` trait has another important limitation: it is not `async`.

You cannot `await` inside a destructor, which is often needed when cleaning up
asynchronous resources like sockets, database connections, or tasks that must
signal completion to another system.

- Learn more:
  <https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_drop.html>
- Available on nightly:
  <https://doc.rust-lang.org/nightly/std/future/trait.AsyncDrop.html>

</details>

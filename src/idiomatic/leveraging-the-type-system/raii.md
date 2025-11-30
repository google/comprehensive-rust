---
minutes: 30
---

# RAII: `Drop` trait

RAII (**R**esource **A**cquisition **I**s **I**nitialization) ties the lifetime
of a resource to the lifetime of a value.

[Rust uses RAII to manage memory](https://doc.rust-lang.org/rust-by-example/scope/raii.html),
and the `Drop` trait allows you to extend this to other resources, such as file
descriptors or locks.

```rust,editable
pub struct File(std::os::fd::RawFd);

impl File {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        // [...]
        Ok(Self(0))
    }

    pub fn read_to_end(&mut self) -> Result<Vec<u8>, std::io::Error> {
        // [...]
        Ok(b"example".to_vec())
    }

    pub fn close(self) -> Result<(), std::io::Error> {
        // [...]
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("example.txt")?;
    println!("content: {:?}", file.read_to_end()?);
    Ok(())
}
```

<details>

- This example shows how easy it is to forget releasing a file descriptor when
  managing it manually. The code as written does not call `file.close()`. Did
  anyone in the class notice?

- To release the file descriptor correctly, `file.close()` must be called after
  the last use — and also in early-return paths in case of errors.

- Instead of relying on the user to call `close()`, we can implement the `Drop`
  trait to release the resource automatically. This ties cleanup to the lifetime
  of the `File` value.

  ```rust,compile_fail
  impl Drop for File {
      fn drop(&mut self) {
          println!("release file descriptor automatically");
      }
  }
  ```

- Note that `Drop::drop` cannot return errors. Any fallible logic must be
  handled internally or ignored. In the standard library, errors returned while
  closing an owned file descriptor during `Drop` are silently discarded:
  <https://doc.rust-lang.org/src/std/os/fd/owned.rs.html#169-196>

- If both `drop()` and `close()` exist, the file descriptor may be released
  twice. To avoid this, remove `close()` and rely solely on `Drop`.

- When is `Drop::drop` called?

  Normally, when the `file` variable in `main` goes out of scope (either on
  return or due to a panic), `drop()` is called automatically.

  If the file is moved into another function, for example `read_all()`, the
  value is dropped when that function returns — not in `main`.

  In contrast, C++ runs destructors in the original scope even for moved-from
  values.

- The same mechanism powers `std::mem::drop`:

  ```rust
  pub fn drop<T>(_x: T) {}
  ```

  You can use it to force early destruction of a value before its natural end of
  scope.

- Insert `panic!("oops")` at the start of `read_to_end()` to show that `drop()`
  still runs during unwinding.

- There are cases where destructors will not run:
  - If a destructor itself panics during unwinding, the program aborts
    immediately.
  - If the object that implements `Drop` is leaked, for example, through
    `std::mem::forget()`. Leaking is safe in Rust.
  - If the program exits with `std::process::exit()` or is compiled with the
    `abort` panic strategy, destructors are skipped.

### More to Explore

The `Drop` trait has another important limitation: it is not `async`.

This means you cannot `await` inside a destructor, which is often needed when
cleaning up asynchronous resources like sockets, database connections, or tasks
that must signal completion to another system.

- Learn more:
  <https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_drop.html>
- There is an experimental `AsyncDrop` trait available on nightly:
  <https://doc.rust-lang.org/nightly/std/future/trait.AsyncDrop.html>

</details>

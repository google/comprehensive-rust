---
minutes: 60
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

- Easy to miss: `file.close()` is never called. Ask the class if they noticed.

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

- Note that `Drop::drop` cannot return a Result. Any fallible logic must be
  handled internally or ignored. In the standard library, errors during FD
  closure inside `Drop` are silently discarded. See the implementation:
  <https://doc.rust-lang.org/src/std/os/fd/owned.rs.html#169-196>

- When is `Drop::drop` called?

  Normally, when the `file` variable in `main` goes out of scope (either on
  return or due to a panic), `drop()` is called automatically.

  If the file is moved into another function, for example `read_all()`, the
  value is dropped when that function returns — not in `main`.

  In contrast, C++ runs destructors in the original scope even for moved-from
  values.

- Demo: insert `panic!("oops")` at the start of `read_to_end()` and run it.
  `drop()` still runs during unwinding.

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

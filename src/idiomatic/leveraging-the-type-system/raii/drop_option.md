# Drop: Option

```rust,editable
struct File(Option<Handle>);

impl File {
    fn open(path: &'static str) -> std::io::Result<Self> {
        Ok(Self(Some(Handle { path })))
    }

    fn write(&mut self, data: &str) -> std::io::Result<()> {
        match &mut self.0 {
            Some(handle) => println!("write '{data}' to file '{}'", handle.path),
            None => unreachable!(),
        }
        Ok(())
    }

    fn close(mut self) -> std::io::Result<&'static str> {
        Ok(self.0.take().unwrap().path)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            println!("automatically closing handle for file: {}", handle.path);
        }
    }
}

struct Handle {
    path: &'static str,
}
impl Drop for Handle {
    fn drop(&mut self) {
        println!("closed handle for file: {}", self.path)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    file.write("hello")?;
    println!("manually closed file: {}", file.close()?);
    Ok(())
}
```

<details>

- In this example we want to let the user call `close()` manually so that errors
  from closing the file can be reported explicitly.

- At the same time we still want RAII semantics: if the user forgets to call
  `close()`, the handle must be cleaned up automatically in `Drop`.

- Wrapping the handle in an `Option` gives us both behaviors. `close()` extracts
  the handle with `take()`, and `Drop` only runs cleanup if a handle is still
  present.

  Demo: remove the `.close()` call and run the code — `Drop` now prints the
  automatic cleanup.

- The main downside is ergonomics. `Option` forces us to handle both the `Some`
  and `None` case even in places where, logically, `None` cannot occur. Rust’s
  type system cannot express that relationship between `File` and its `Handle`,
  so we handle both cases manually.

## More to explore

Instead of `Option` we could use
[`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html),
which suppresses automatic destruction by preventing Rust from calling `Drop`
for the value; you must handle teardown yourself.

The [_scopeguard_ example](./scope_guard.md) on the previous slide shows how
`ManuallyDrop` can replace `Option` to avoid handling `None` in places where the
value should always exist.

In such designs we typically track the drop state with a separate flag next to
the `ManuallyDrop<Handle>`, which lets us track whether the handle has already
been manually consumed.

</details>

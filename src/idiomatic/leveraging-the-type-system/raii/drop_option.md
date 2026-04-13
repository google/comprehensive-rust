---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Drop: Option

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
struct File(Option<Handle>);

impl File {
    fn open(path: &'static str) -> std::io::Result<Self> {
        Ok(Self(Some(Handle { path })))
    }

    fn write(&mut self, data: &str) -> std::io::Result<()> {
        // We have to go through the `Option` to get the `Handle`
        // before we can use it.
        let handle = self.0.as_ref().unwrap();
        println!("write '{data}' to file '{}'", handle.path);
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let handle = self.0.take().unwrap();
        handle.close();
    }
}

struct Handle {
    path: &'static str,
}

impl Handle {
    fn close(self) {
        println!("Closing {}", self.path);
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    file.write("hello")?;
    Ok(())
}
```

<details>

- In this example we want to call `close` on the inner `Handle` in our `Drop`
  impl, but `close` requires ownership of the `Handle`. We can't do this
  normally, because we don't get ownership of the `File` object in `drop`, and
  therefore can't move out of the field.

- Wrapping the handle in an `Option` gives us a way to move out of the field
  through a mutable reference.

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

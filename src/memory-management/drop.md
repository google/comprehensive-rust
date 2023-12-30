---
minutes: 10
---

# The `Drop` Trait

Values which implement [`Drop`][1] can specify code to run when they go out of
scope:

```rust,editable
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");
}
```

<details>

- Note that `std::mem::drop` is not the same as `std::ops::Drop::drop`.
- Values are automatically dropped when they go out of scope.
- When a value is dropped, if it implements `std::ops::Drop` then its
  `Drop::drop` implementation will be called.
- All its fields will then be dropped too, whether or not it implements `Drop`.
- `std::mem::drop` is just an empty function that takes any value. The
  significance is that it takes ownership of the value, so at the end of its
  scope it gets dropped. This makes it a convenient way to explicitly drop
  values earlier than they would otherwise go out of scope.
  - This can be useful for objects that do some work on `drop`: releasing locks,
    closing files, etc.

Discussion points:

- Why doesn't `Drop::drop` take `self`?
  - Short-answer: If it did, `std::mem::drop` would be called at the end of the
    block, resulting in another call to `Drop::drop`, and a stack overflow!
- Try replacing `drop(a)` with `a.drop()`.

</details>

[1]: https://doc.rust-lang.org/std/ops/trait.Drop.html

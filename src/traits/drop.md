# The `Drop` Trait

Values which implement [`Drop`][1] can specify code to run when they go out of scope:

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

Discussion points:

* Why doesn't `Drop::drop` take `self`?
    * Short-answer: If it did, `std::mem::drop` would be called at the end of
        the block, resulting in another call to `Drop::drop`, and a stack
        overflow!
* Try replacing `drop(a)` with `a.drop()`.

</details>

[1]: https://doc.rust-lang.org/std/ops/trait.Drop.html

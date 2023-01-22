# The `Drop` Trait

Values which implement `Drop` can specify code to run when they go out of scope:

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

* `drop` is called automatically, but it can be called manually like in this example.
* If it was called manually, it won't be called at the end of the scope for the second time.
* Calling `drop` earlier manually can be useful for objects that do some work on `drop`: lock guards, files.
    * It is useful, but not hugely useful as you can always make custom scope by enclosing your code in a block.

</details>

---
minutes: 2
---

# Clone

Sometimes you _want_ to make a copy of a value. The `Clone` trait accomplishes
this.

```rust,editable
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
}
```

<details>

- The idea of `Clone` is to make it easy to spot where heap allocations are
  occurring. Look for `.clone()` and a few others like `Vec::new` or `Box::new`.

- It's common to "clone your way out" of problems with the borrow checker, and
  return later to try to optimize those clones away.

- `clone` generally performs a deep copy of the value, meaning that if you e.g.
  clone an array, all of the elements of the array are cloned as well.

- The behavior for `clone` is user-defined, so it can perform custom cloning
  logic if needed.

</details>

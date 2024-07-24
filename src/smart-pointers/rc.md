---
minutes: 5
---

# `Rc`

[`Rc`][1] is a reference-counted shared pointer. Use this when you need to refer
to the same data from multiple places:

```rust,editable
use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}
```

- See [`Arc`][2] and [`Mutex`][3] if you are in a multi-threaded context.
- You can _downgrade_ a shared pointer into a [`Weak`][4] pointer to create
  cycles that will get dropped.

[1]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[2]: ../concurrency/shared-state/arc.md
[3]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[4]: https://doc.rust-lang.org/std/rc/struct.Weak.html

<details>

- `Rc`'s count ensures that its contained value is valid for as long as there
  are references.
- `Rc` in Rust is like `std::shared_ptr` in C++.
- `Rc::clone` is cheap: it creates a pointer to the same allocation and
  increases the reference count. Does not make a deep clone and can generally be
  ignored when looking for performance issues in code.
- `make_mut` actually clones the inner value if necessary ("clone-on-write") and
  returns a mutable reference.
- Use `Rc::strong_count` to check the reference count.
- `Rc::downgrade` gives you a _weakly reference-counted_ object to create cycles
  that will be dropped properly (likely in combination with `RefCell`).

</details>

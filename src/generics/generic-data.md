---
minutes: 15
existing course material:
- generics/data-types.md
- generics/methods.md
---

# Generic Data types

# Generic Data Types

You can use generics to abstract over the concrete field type:

```rust,editable
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
}
```

<details>

* Try declaring a new variable `let p = Point { x: 5, y: 10.0 };`.

* Fix the code to allow points that have elements of different types.

</details>
# Generic Methods

You can declare a generic type on your `impl` block:

```rust,editable
#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0  // + 10
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}
```

<details>

* *Q:* Why `T` is specified twice in `impl<T> Point<T> {}`? Isn't that redundant?
    * This is because it is a generic implementation section for generic type. They are independently generic.
    * It means these methods are defined for any `T`.
    * It is possible to write `impl Point<u32> { .. }`.
      * `Point` is still generic and you can use `Point<f64>`, but methods in this block will only be available for `Point<u32>`.

</details>

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
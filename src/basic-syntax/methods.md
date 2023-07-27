# Methods

Methods are functions associated with a type. The `self` argument of a method is
an instance of the type it is associated with:

```rust,editable
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
```

* We will look much more at methods in today's exercise and in tomorrow's class.

<details>

- Add a static method called `Rectangle::new` and call this from `main`:

    ```rust,editable,compile_fail
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    ```

- While _technically_, Rust does not have custom constructors, static methods are commonly used to initialize structs (but don't have to).
  The actual constructor, `Rectangle { width, height }`, could be called directly. See the [Rustnomicon](https://doc.rust-lang.org/nomicon/constructors.html).

- Add a `Rectangle::square(width: u32)` constructor to illustrate that such static methods can take arbitrary parameters.

</details>

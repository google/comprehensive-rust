# Methods

Rust allows you to associate functions with your new types. You do this with an
`impl` block:

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

<details>

Key Points:
* It can be helpful to introduce methods by comparing them to functions.
  * Methods are called on an instance of a type (such as a struct or enum), the first parameter represents the instance as `self`.
  * Developers may choose to use methods to take advantage of method receiver syntax and to help keep them more organized. By using methods we can keep all the implementation code in one predictable place.
* Point out the use of the keyword `self`, a method receiver. 
  * Show that it is an abbreviated term for `self:&Self` and perhaps show how the struct name could also be used. 
  * Explain that `Self` is a type alias for the type the `impl` block is in and can be used elsewhere in the block.
  * Note how `self` is used like other structs and dot notation can be used to refer to individual fields.
  * This might be a good time to demonstrate how the `&self` differs from `self` by modifying the `area` method to use `self`.
* We describe the distinction between method receivers next.
   
</details>

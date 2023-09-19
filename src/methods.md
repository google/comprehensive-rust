# Methods

Rust allows you to associate functions with your new types. You do this with an
`impl` block:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
```

<details>

Key Points:
* It can be helpful to introduce methods by comparing them to functions.
  * Methods are called on an instance of a type (such as a struct or enum), the first parameter represents the instance as `self`.
  * Developers may choose to use methods to take advantage of method receiver syntax and to help keep them more organized. By using methods we can keep all the implementation code in one predictable place.
* Point out the use of the keyword `self`, a method receiver.
  * Show that it is an abbreviated term for `self: Self` and perhaps show how the struct name could also be used.
  * Explain that `Self` is a type alias for the type the `impl` block is in and can be used elsewhere in the block.
  * Note how `self` is used like other structs and dot notation can be used to refer to individual fields.
  * This might be a good time to demonstrate how the `&self` differs from `self` by modifying the code and trying to run say_hello twice.
* We describe the distinction between method receivers next.

</details>

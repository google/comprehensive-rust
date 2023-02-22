# Field Shorthand Syntax

If you already have variables with the right names, then you can create the
struct using a shorthand:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
```

<details>

*  The `new` function could be written using `Self` as a type, as it is interchangeable with the struct type name

```rust,ignore
impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
```

* Methods are defined in the `impl` block.
* Use struct update syntax to define a new structure using `peter`. Note that the variable `peter` will no longer be accessible afterwards.
* Use `{:#?}` when printing structs to request the `Debug` representation.

</details>

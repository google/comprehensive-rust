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


The `new` function could be written using `Self` as a type, as it is interchangeable with the struct type name

```rust,ignore
impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
```
    

<details>

* Point out the syntax that is used inside the `new` function. To show a comparison you can also rewrite the struct definition using non-shorthand syntax.
* The `impl` block is where the boilerplate functions are defined. Later slides talk about methods in more detail.
* Use function update syntax to define a new structure using `peter`. Note that the variable `peter` will no longer be accessible afterwards.
* Note, you can use `{:#?}` while printing structs to change the readability of the output.
</details>

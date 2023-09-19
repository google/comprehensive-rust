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

     ```rust,editable
     #[derive(Debug)]
     struct Person {
         name: String,
         age: u8,
     }
     impl Person {
         fn new(name: String, age: u8) -> Self {
             Self { name, age }
         }
     }
     ```    
* Implement the `Default` trait for the struct. Define some fields and use the default values for the other fields.

     ```rust,editable
     #[derive(Debug)]
     struct Person {
         name: String,
         age: u8,
     }
     impl Default for Person {
         fn default() -> Person {
             Person {
                 name: "Bot".to_string(),
                 age: 0,
             }
         }
     }
     fn create_default() {
         let tmp = Person {
             ..Person::default()
         };
         let tmp = Person {
             name: "Sam".to_string(),
             ..Person::default()
         };
     }
     ```

* Methods are defined in the `impl` block.
* Use struct update syntax to define a new structure using `peter`. Note that the variable `peter` will no longer be accessible afterwards.
* Use `{:#?}` when printing structs to request the `Debug` representation.

</details>

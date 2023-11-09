---
minutes: 10
existing course material:
- structs.md
- structs/field-shorthand.md
---

<!-- NOTES:
Overview of type names, naming conventions, field shorthand, `..` notation
-->
# Named Structs

# Structs

Like C and C++, Rust has support for custom structs:

```rust,editable
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}
```

<details>

Key Points:

* Structs work like in C or C++.
  * Like in C++, and unlike in C, no typedef is needed to define a type.
  * Unlike in C++, there is no inheritance between structs.
* Methods are defined in an `impl` block, which we will see in following slides.
* This may be a good time to let people know there are different types of structs.
  * Zero-sized structs (e.g. `struct Foo;`) might be used when implementing a trait on some type but don’t have any data that you want to store in the value itself.
  * The next slide will introduce Tuple structs, used when the field names are not important.
* The syntax `..peter` allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element.

</details>
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

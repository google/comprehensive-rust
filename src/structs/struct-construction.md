# Construction

Unlike in C++ or Java, there is no "constructor" in Rust for structs, all
construction is done using struct construction syntax.

It is convention, however, to either implement the `Default` trait, or create a
method called "new" that has no receiver (ie a "static" function).

```rust,editable
struct Person {
    name: String,
    age: u8,
}

impl Person {
  pub fn new(name: String, age: u8) -> Person {
    Person {
      name,
      age
    }
  }

  pub fn new_birth(name: String) -> Person {
    Self::new(name, 0)
  }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 23);

    println!("{} is {} years old", peter.name, peter.age);
}
```

<details>

* Mention the `Self` static scope accessor, it allows you to access any method of a struct.

* In fact, dot method call syntax is just syntactic sugar, you can even access methods with &self receiver parameters by explicitly passing structs in to the first parameter, eg `Person::display(&peter)` if it had such a method `display(&self)`.

* Mention it is likely better to take string references and clone them in the construction methods, but we wanted to keep the example simple and consistent with others.

</details>
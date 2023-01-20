# Structs

Like C and C++, Rust has support for custom structs:

```rust,editable
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    println!("{} is {} years old", peter.name, peter.age);
}
```

<details>
Key Points: 

* Structs work similar as they do in other languages, but note there is no `typedef` keyword or inheritance between structs. 
* Methods are defined in an `impl` block, which we will see in following slides.
* This may be a good time to let people know there are different types of structs. The next slide will introduce Tuple structs. Here you can define a unit-like struct.
    * Unit-like structs can be defined with `struct (name);`. There is no need for curly brackets and parentheses.
    * You may use this type of struct when implementing a trait on some type but don’t have any data that you want to store in the value itself. 

</details>

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
    
* Structs work similar as they do in other languages.
    * Note there is no `typedef` keyword or inheritance between structs.
    * Methods are defined in an `impl` block, which we'll see in following slides.
* We can access the structs fields with the dot notation.
* Using short-hand syntax, we can initiate the fields with similar-named variables. 
    * This is where you can define the `age` variable on its own line. 
* There are multiple types of syntax for structs in Rust. Here are can demonstrate a unit-like struct and in the next slide we show a Tuple Struct.
    * Unit-like structs can be defined with `struct (name);`. There is no need for curly brackets and parentheses.
    * You may use this type of struct when to implement a trait on some type but don’t have any data that you want to store in the value itself. 

</details>

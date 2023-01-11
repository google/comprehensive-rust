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

* The top of the program is how we define a struct, the main function creates a struct, and then at the bottom we access the fields.
* Structs work similar as they do in other languages, but note there is also no typedef keyword or inheritance between structs. Also note there are no functions. The implementation is separate.
* We can access the structs fields with the dot notation 
* Using short-hand syntax, we can initiate the fields with similar-named variables.  [define age variable on it’s own line] 
* There are multiple types of syntax for structs in Rust
    * Unit_ like struct, can Define with “struct (name);” and no need for curly brackets and parentheses.
    * Use: when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 
    * next lets see a Tuple struct


</details>
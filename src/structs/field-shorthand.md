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
 Key Points: 
    
* This slide illustrates short-hand syntax, by initiating the fields in a struct with similar-named variables. 
* The `impl` block is where the boilerplate functions are defined. Later slides talk about methods in more detail.
* This might be a good time to demonstrate function update syntax while pointing out the effects that the borrowing mechanism has on the original data sources.
* Note, you can use `{:#?}` while printing structs to change the readability of the output.
   
</details>

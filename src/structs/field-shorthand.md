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

* “Impl”: This is where we define the new() function and add other boilerplate functions. 
* Field Shorthand Syntax: Previously we used a field name with a colon, here [demonstrating the fss] when variable name is the same as the field name
* Function update syntax: 
    * It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. 
    * [let dad =  Person {age:57, .. peter};]
* It’s useful to have output that’s a bit easier to read  [use {:#?}  and re-run]

</details>
# Setting Default values in Structs

Deriving defaults
```
#[derive(Debug, Default)]
```

or by creating a Default implementation 

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Default for Person {
  fn default() -> Person { Person { name:"Alice".to_string(), age:54 } }
}
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
    let tmp = Person {age:12, .. Default::default()};
    println!("{tmp:?}");
    let tmp = Person { name:"Sam".to_string(), .. Default::default()};
    println!("{tmp:?}");
}
```


# Moves in Function Calls

When you pass a value to a function, the value is assigned to the function
parameter. This transfers ownership:

```rust,editable
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
```

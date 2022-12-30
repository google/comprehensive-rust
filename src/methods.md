# Methods

러스트에서 선언된 타입에 대해 `impl`블록에 함수를 선언하여 메서드를 연결할 수 있습니다:
> Rust allows you to associate functions with your new types. You do this with an
> `impl` block:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
```


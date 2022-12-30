# Structs

C/C++과 마찬가지로 러스트는 커스텀 구조체를 지원합니다:
> Like C and C++, Rust has support for custom structs:

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

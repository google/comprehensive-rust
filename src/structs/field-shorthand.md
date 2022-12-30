# Field Shorthand Syntax

구조체 필드와 동일한 이름의 변수가 있다면 아래와 같이 _짧은 문법_ 으로 구조체를 생성할 수 있습니다:
> If you already have variables with the right names, then you can create the
> struct using a shorthand:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age } // <-- 이부분
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
```


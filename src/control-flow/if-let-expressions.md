# `if let` expressions

`if let`문을 사용하면 값을 패턴매칭에 사용할 수 있습니다:
> If you want to match a value against a pattern, you can use `if let`:

```rust,editable
fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}
```

러스트의 [패턴매칭](../pattern-matching.md)을 참조하세요
> See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

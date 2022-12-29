# Variables

러스트는 정적 타이핑을 통해 타입 세이프티(타입 안전)을 제공합니다. 
변수 바인딩은 기본적으로 불변(immutable)합니다.
> Rust provides type safety via static typing. Variable bindings are immutable by default:

```rust,editable
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
```

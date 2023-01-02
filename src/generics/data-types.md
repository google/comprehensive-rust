# Generic Data Types

제너릭을 사용하여 구체적인 타입을 추상화할 수 있습니다:
> You can use generics to abstract over the concrete field type:

```rust,editable
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
}
```

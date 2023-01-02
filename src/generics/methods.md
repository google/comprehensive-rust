# Generic Methods

`impl` 블록에서도 제너릭 타입을 선언할 수 있습니다:
> You can declare a generic type on your `impl` block:

```rust,editable
#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0  // + 10
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}
```

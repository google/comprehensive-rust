# `Add`, `Mul`, ...
연산자 오버로드는 `std::ops` 트레이트을 통해 구현됩니다:
> Operator overloading is implemented via traits in `std::ops`:

```rust,editable
#[derive(Debug, Copy, Clone)]
struct Point { x: i32, y: i32 }

// + 연산자 오버로딩
impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}
```

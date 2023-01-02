# Default Methods

트레이트는 다른 트레이트 메서드에서의 동작을 구현할 수 있습니다.
> Traits can implement behavior in terms of other trait methods:

```rust,editable
trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}
```
---
역주
- Equals 트레이트에서 equal은 선언만 되어있고 not_equal에서 이를 호출하고 있습니다. (추상 메서드)
- impl에서 equal 메서드를 정의하고 있어서 Centimeter에서의 equal 메서드를 구현되어 있습니다.(추상 메서드 구현)


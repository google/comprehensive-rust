# Polygon Struct


우리는 몇개의 꼭지점을 가진 `다각형`을 표현하는 구조체를 만들 것입니다. 

아래 코드를 <https://play.rust-lang.org/>에 복사해서 테스트가 통과하도록 빠진 메서드를 구현하시면 됩니다: 
> We will create a `Polygon` struct which contain some points. Copy the code below
> to <https://play.rust-lang.org/> and fill in the missing methods to make the
> tests pass:

```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include points-polygons.rs:Point}}
    // add fields
}

{{#include points-polygons.rs:Point-impl}}
    // add methods
}

{{#include points-polygons.rs:Polygon}}
    // add fields
}

{{#include points-polygons.rs:Polygon-impl}}
    // add methods
}

{{#include points-polygons.rs:Circle}}
    // add fields
}

{{#include points-polygons.rs:Circle-impl}}
    // add methods
}

{{#include points-polygons.rs:Shape}}

{{#include points-polygons.rs:unit-tests}}

#[allow(dead_code)]
fn main() {}
```


---
역주
- 구현체의 +(Add), -(Sub)까지 구현하게 만들었는데 난이도가 꽤 높습니다
- shape와 관련된 impl도 구현해야합니다. 
- 일단 실행해서 컴파일러 오류를 잡아가는 TDD로 진행해야 그나마(...) 수월하네요
<details>
<summary>힌트- 포인트 연산 함수, shape from 함수</summary>

```rust
impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}
```

</details>
<details>
<summary>힌트- 구현함수 설명</summary>

- point.magnitude : (0,0)과 point의 거리
- point.dist: 입력받은 포인트와 point 사이의 거리
- shape.circumference: 테두리 길이(다각형), 둘레(원)
- 
</details>
</details>
<details>
<summary>힌트- 참조할 수식함수 설명</summary>

- 공식은 생략합니다(..)
- [제곱연산](https://docs.rs/num/0.2.1/num/pow/index.html)
- [f64::루트](https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt)
- PI : `std::f64::consts::PI` 
</details>
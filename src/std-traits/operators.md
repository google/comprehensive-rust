---
minutes: 10
---

# Operators

Operator overloading is implemented via traits in [`std::ops`][1]:

```rust,editable
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}
```

<details>

Discussion points:

- You could implement `Add` for `&Point`. In which situations is that useful?
  - Answer: `Add:add` consumes `self`. If type `T` for which you are overloading
    the operator is not `Copy`, you should consider overloading the operator for
    `&T` as well. This avoids unnecessary cloning on the call site.
- Why is `Output` an associated type? Could it be made a type parameter of the
  method?
  - Short answer: Function type parameters are controlled by the caller, but
    associated types (like `Output`) are controlled by the implementer of a
    trait.
- You could implement `Add` for two different types, e.g.
  `impl Add<(i32, i32)> for Point` would add a tuple to a `Point`.

</details>

[1]: https://doc.rust-lang.org/std/ops/index.html

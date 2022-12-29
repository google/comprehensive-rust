# Copying and Cloning

의미구조(Semantics)가 이동할때, 특정 타입은 기본적으로 복사됩니다:
> While move semantics are the default, certain types are copied by default:

```rust,editable
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");
}
```

이러한 유형들은 `Copy` 트레이트를 구현합니다. 

직접 만든 타입들도 `Copy`트레이트를 구현하여 의미복사를 할 수 있습니다:
> These types implement the `Copy` trait.
> 
> You can opt-in your own types to use copy semantics:

```rust,editable
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

* 할당 후, `p1`와 `p2`는 자신의 데이터를 소유합니다.
* 명시적으로 `p1.clone()`를 사용하여 데이터를 복사할 수 있습니다.
> * After the assignment, both `p1` and `p2` own their own data.
> * We can also use `p1.clone()` to explicitly copy the data.

---
역주
- `#[derive(Copy, Clone, Debug)]`를 통해 Point 구조체가 Copy되도록 함
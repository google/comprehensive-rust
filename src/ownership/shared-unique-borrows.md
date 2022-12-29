# Shared and Unique Borrows

러스트는 값을 빌려오는 방법에 대한 제약이 존재합니다:

* 한번에 하나 이상의 `&T` 값을 가질 수 있습니다.(1~N개의 불변 변수)
* 또는 
* 정확히 하나의 `&mut T` 값을 가질 수 있습니다.(1개의 가변 변수)
> Rust puts constraints on the ways you can borrow values:
> 
> * You can have one or more `&T` values at any given time, _or_
> * You can have exactly one `&mut T` value.

```rust,editable,compile_fail
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}
```
---
역주
- 둘 중 하나만 됩니다. 따라서 위 소스는 내부 스코프에서 1개의 불변변수, 1개의 가변변수가 존재하게 되서 오류입니다.
# `while let` expressions

`if`와 동일하게 `while let`구문 역시 패턴매칭에 사용 할 수 있습니다.
> Like with `if`, there is a `while let` variant which repeatedly tests a value
> against a pattern:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
```

`v.iter()`가 반환한 반복자는 `next()`가 호출될 때마다 `Option<i32>`를 반환합니다.
이는 `None`이 호출되기 전까지 `Some(x)`를 반환한다는 의미로 결과적으로 `while let`은 반복자의 모든 아이템을 반복하도록 하게 해줍니다.

러스트의 [패턴매칭](../pattern-matching.md)을 참조하세요

> Here the iterator returned by `v.iter()` will return a `Option<i32>` on every
> call to `next()`. It returns `Some(x)` until it is done, after which it will
> return `None`. The `while let` lets us keep iterating through all items.
> 
> See [pattern matching](../pattern-matching.md) for more details on patterns in Rust.

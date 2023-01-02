# Monomorphization[1]

제너릭 코드는 호출부에서 비 제너릭 코드로 전환됩니다:
> Generic code is turned into non-generic code based on the call sites:

```rust,editable
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

위 코드는 아래와 같이 동작합니다.
> behaves as if you wrote

```rust,editable
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

이것은 코스트가 들지 않는 추상화[^역주1]입니다: 추상화 없이 직접 코딩한 것과 정확히 같은 결과입니다.
> This is a zero-cost abstraction: you get exactly the same result as if you had
> hand-coded the data structures without the abstraction.

---
역주

[1]: https://en.wikipedia.org/wiki/Monomorphization
[^역주1]: 제너릭과 같이 런타임 코스트 없이 컴파일 코스트만으로 동작하는 추상화 컨셉입니다.
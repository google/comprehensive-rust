# `impl Trait`

트레이트 바운드와 유사하게 `impl`트레이트 문법은 함수의 인자와 반환값에 적용 가능합니다:
> Similar to trait bounds, an `impl Trait` syntax can be used in function
> arguments and return values:

```rust,editable
use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}
```
* `impl` 트레이트는 터보피쉬문법(`::<>`)에는 사용할 수 없습니다.
* `impl` 트레이트는 익명타입과 사용할 수 있습니다.
> * `impl Trait` cannot be used with the `::<>` turbo fish syntax.
> * `impl Trait` allows you to work with types which you cannot name.

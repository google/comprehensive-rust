# Implicit Conversions

러스트는 [C++과 다르게][3] 타입 간 _암묵적 변환_을 자동으로 적용하지 않습니다.
아래 예시를 확인해 보세요:
> Rust will not automatically apply _implicit conversions_ between types ([unlike
> C++][3]). You can see this in a program like this:

```rust,editable,compile_fail
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}
```

러스트의 정수형 타입은 모두 [`From<T>`][1] 와 [`Into<T>`][2] 트레이트를 구현하여 둘 사이를 변환 할 수 있습니다.
`From<T>` 트레이트는 단일 `from()` 메서드를 가지고 있고, `Into<T>`트레이트는 단일 `into()`메서드를 가지고 있습니다.
이러한 트레이트를 구현하는 것은 타입이 다른 타입으로 변환하는 것을 표현하는 방법입니다.

> The Rust integer types all implement the [`From<T>`][1] and [`Into<T>`][2]
> traits to let us convert between them. The `From<T>` trait has a single `from()`
> method and similarly, the `Into<T>` trait has a single `into()` method.
> Implementing these traits is how a type expresses that it can be converted into
> another type.

표준 라이브러리에는 `From<i8> for i16`가 구현되어 있는데 이것은 우리가 `i8` 타입의 변수 `x`를 `i16::from(x)`를 호출하여 `i16`타입으로 변환할 수 있다는 의미입니다. 
(혹은 더 간단하게 `x.into()`를 사용합니다.)
`From<i8> for i16`의 구현은 자동으로 `Into<i16> for i8`를 생성하기 때문입니다.

> The standard library has an implementation of `From<i8> for i16`, which means
> that we can convert a variable `x` of type `i8` to an `i16` by calling 
> `i16::from(x)`. Or, simpler, with `x.into()`, because `From<i8> for i16`
> implementation automatically create an implementation of `Into<i16> for i8`.

1. 위 예제코드를 실행해서 발생한 오류를 확인해 보세요
2. `into()` 메서드를 사용하여 코드를 수정하세요 
3. `x`와 `y`를 `f32'이나 'bool', 'i128' 등으로 바꿔서 해당 타입들로 변환이 되는지 확인해보세요 
  - 작은 사이즈 타입에서 큰 사이즈로 변경해보시고 그 반대로도 해보세요 
  - [표준 라이브러리 문서][1]에서 시도해 본 케이스가 구현되어 있는지 확인해 보세요.

> 1. Execute the above program and look at the compiler error.
> 2. Update the code above to use `into()` to do the conversion.
> 3. Change the types of `x` and `y` to other things (such as `f32`, `bool`,
   `i128`) to see which types you can convert to which other types. Try
   converting small types to big types and the other way around. Check the
   [standard library documentation][1] to see if `From<T>` is implemented for
   the pairs you check.

[1]: https://doc.rust-lang.org/std/convert/trait.From.html
[2]: https://doc.rust-lang.org/std/convert/trait.Into.html
[3]: https://en.cppreference.com/w/cpp/language/implicit_conversion

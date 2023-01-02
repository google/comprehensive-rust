# String

[`String`][1]은 힙에 할당된 확장가능한 표준 UTF-8 문자열 버퍼입니다.
> [`String`][1] is the standard heap-allocated growable UTF-8 string buffer:

```rust,editable
fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());
}
```

`String`은 [`Deref<Target = str>`][2]을 구현합니다.
이는 , `String`에서 모든 `str`관련 메서드를 호출 할 수 있다는 의미 입니다.
> `String` implements [`Deref<Target = str>`][2], which means that you can call all
> `str` methods on a `String`.

[1]: https://doc.rust-lang.org/std/string/struct.String.html
[2]: https://doc.rust-lang.org/std/string/struct.String.html#deref-methods-str

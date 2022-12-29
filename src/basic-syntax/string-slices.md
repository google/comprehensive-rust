# `String` vs `str`

이제 러스트의 두가지 문자열 타입에 대해서 이해해 보겠습니다.:
> We can now understand the two string types in Rust:

```rust,editable
fn main() {
    let s1: &str = "Hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}
```
러스트 용어:
* `&str` 타입은 문자열 슬라이스의 불변참조입니다.
* `String` 타입은 가변 문자열 버퍼입니다.

> Rust terminology:
> 
> * `&str` an immutable reference to a string slice.
> * `String` a mutable string buffer.

---
역주
- str은 문자 리터럴, &은 참조 타입입니다. 
# `match` expressions

마치 `if let`를 여러개 적용한 것과 같이 `match`키워드는 하나 이상의 패턴을 찾는데 사용됩니다.
> The `match` keyword is used to match a value against one or more patterns. In
> that sense, it works like a series of `if let` expressions:

```rust,editable
fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}
```


`if let`처럼 각 매치 암(match arm)의 마지막 표현식이 타입이 되며 모든 암은 동일한 타입이어야 합니다. 위의 예제에서 타입은 `()`(반환값 없음)입니다.

러스트의 [패턴매칭](../pattern-matching.md)을 참조하세요

Like `if let`, each match arm must have the same type. The type is the last
expression of the block, if any. In the example above, the type is `()`.
> 
> See [pattern matching](../pattern-matching.md) for more details on patterns in
> Rust.

---
역주
- 원문이 좀 난해한데, 예시에서 각 암의 타입은 리턴 없음(출력만 하고 끝)입니다(러스트에서`()` 라고 합니다. [`함수` 소스 중간 주석 참조](../basic-syntax/functions.md))
- 각 암도 블록`{}`으로 표현가능하고 이 역시 함수처럼 마지막 표현식으로 리턴 시킬 수 있는데 이를 각 암의 타입으로 한다는 의미. 
- 예제를 여러 반환식으로 테스트 해보시기 바랍니다.
# Test Modules

단위 테스트는 종종 중첩 모듈 안에 존재합니다. [플레이그라운드](https://play.rust-lang.org/)에서 실행하시기 바랍니다)  
--> 단위 테스트는 모듈로 분리해서 선언 가능합니다.
> Unit tests are often put in a nested module (run tests on the
> [Playground](https://play.rust-lang.org/))

```rust,editable
fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}
```


* 개별화된 헬퍼모듈을 통해 단위테스트를 수행할 수 있습니다.
* `#[cfg(test)]` 속성은 오직 `cargo test` 커맨드 실행인 경우에만 동작합니다.
> * This lets you unit test private helpers.
> * The `#[cfg(test)]` attribute is only active when you run `cargo test`.


# Catching the Stack Unwinding

기본적으로, 패닉이 발생하면 스택은 해제됩니다. 스택 해제는 다음과 같이 캐치가 가능합니다: 
> By default, a panic will cause the stack to unwind. The unwinding can be caught:

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    println!("hello!");
});
assert!(result.is_ok());

let result = panic::catch_unwind(|| {
    panic!("oh no!");
});
assert!(result.is_err());
```

* 이것은 단일 요청이 크래시 되더라도 계속 실행되야 하는 서버에 유용합니다.
*  `Cargo.toml`설정파일에 `panic = abort`을 설정하면 동작하지 않습니다.

> * This can be useful in servers which should keep running even if a single
>   request crashes.
> * This does not work if `panic = abort` is set in your `Cargo.toml`.

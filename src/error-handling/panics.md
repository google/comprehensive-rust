# Panics

러스트는 런타임에서 치명적인 오류를 만나면 패닉을 발생할 것입니다:
> Rust will trigger a panic if a fatal error happens at runtime:

```rust,editable,should_panic
fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}
```

* 패닉은 복구할 수 없고 예상치 못한 오류입니다.
  * 패닉은 프로그램 버그의 증상입니다.
* 충돌(크래시)를 허용하지 않아야 하는 경우 패닉을 유발하지 않는(non-panicking) API를 사용합니다.(`Vec::get` 등)

> * Panics are for unrecoverable and unexpected errors.
>   * Panics are symptoms of bugs in the program.
> * Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.

---
역주 
- 위 예제에서 v[100]을 v.get(100) 으로 대체해보세요
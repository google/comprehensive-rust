# Health Statistics

{{#include ../../../third_party/rust-on-exercism/health-statistics.md}}

아래 코드를 <https://play.rust-lang.org/>에 복사해서 빠진 메서드를 구현하시면 됩니다: 
> Copy the code below to <https://play.rust-lang.org/> and fill in the missing
> methods:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include ../../../third_party/rust-on-exercism/health-statistics.rs}}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.weight(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}
```

---
역주
- 딱히 난이도가 없는 문제라 힌트는 없습니다. 
- 플레이그라운드 기준 warning이 좀 있는데 테스트나 실행은 정상이네요
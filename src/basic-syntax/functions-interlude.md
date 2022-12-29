# Function Overloading

오버로딩은 지원되지 않습니다: 

* 개별함수는 단일 구현만 갖습니다. 
  * 항상 고정된 수의 매개변수만 갖습니다. 
  * 항상 고정된 단일 타입 집합을 사용합니다. 
* 매개변수의 기본 값은 지원되지 않습니다. 
  * 모든 호출부에서는 동일한 수의 인자를 설정해야합니다. 
  * 대안으로 매크로를 사용하기도 합니다. 

하지만, 함수의 매개변수는 제너릭을 적용할 수 있습니다. 

> Overloading is not supported:
> 
> * Each function has a single implementation:
>   * Always takes a fixed number of parameters.
>   * Always takes a single set of parameter types.
> * Default values are not supported:
>   * All call sites have the same number of arguments.
>   * Macros are sometimes used as an alternative.
> 
> However, function parameters can be generic:

```rust,editable
// 제너릭
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}
```

---
역주
- [std::process::id](https://doc.rust-lang.org/std/process/fn.id.html): OS관련 프로세스 ID를 반환하는 함수입니다.
- for js developer: `::` 표현은 쉽게 말해 static 함수 호출입니다.
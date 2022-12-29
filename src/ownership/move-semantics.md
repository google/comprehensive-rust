# Move Semantics

(변수의)할당은 변수 간 소유권을 이동시킵니다:
> An assignment will transfer ownership between variables:

```rust,editable
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```
* s1을 s2에 할당하여 소유권을 이전시킵니다.
* 데이터는 s1에서 _이동_됩니다. 따라서 s1은 더이상 접근 할 수 없습니다.
* `s1`의 스코프가 종료되면 아무 일도 없습니다: 아무런 소유권이 없습니다.
* `s2`의 스코프가 종료되면 문자열 데이터는 해제됩니다.
* 값(데이터)의 소유권을 갖는 변수는 항상 _정확히_ 하나 입니다.

> * The assignment of `s1` to `s2` transfers ownership.
> * The data was _moved_ from `s1` and `s1` is no longer accessible.
> * When `s1` goes out of scope, nothing happens: it has no ownership.
> * When `s2` goes out of scope, the string data is freed.
> * There is always _exactly_ one variable binding which owns a value.

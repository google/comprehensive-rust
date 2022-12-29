# Dangling References

러스트는 댕글링 참조를 금지합니다:
> Rust will statically forbid dangling references:

```rust,editable,compile_fail
fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    } // 스코프가 끝나서 x는 삭제되고 ref_x도 참조 해제 됩니다.(댕글링 참조)
    println!("ref_x: {ref_x}");
}
```

* 참조형 값(참조)는 변수로부터 값을 "빌리는 것"을 말합니다. 
* 러스트는 모든 참조의 수명을 추적하여 참조가 충분히 오래 살아있음(참조가 사용될때까지)을 보장합니다. 
* 소유권 부분에서 "빌림"에 대해 좀 더 많은 것을 다룹니다.
> * A reference is said to "borrow" the value it refers to.
> * Rust is tracking the lifetimes of all references to ensure they live long
>   enough.
> * We will talk more about borrowing when we get to ownership.

---
역주
- 댕글링 참조: 참조 포인터가 더이상 유효하지 않은 값을 가르키게 되는 경우
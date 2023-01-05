# Compound Types

|                 | 타입 Types                    | 예시 Literals                     |
|-----------------|-------------------------------|-----------------------------------|
| 배열<br/>Arrays | `[T; N]`                      | `[20, 30, 40]`, `[0; 3]`          |
| 튜플<br/>Tuples | `()`, `(T,)`, `(T1, T2)`, ... | `()`, `('x',)`, `('x', 1.2)`, ... |
배열 선언과 접근:
> Array assignment and access:

```rust,editable
fn main() {
    let mut a: [i8; 10] = [42; 10]; //타입과 사이즈 선언
    a[5] = 0;
    println!("a: {:?}", a);
}
```

튜플 선언과 접근:
> Tuple assignment and access:

```rust,editable
fn main() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}
```

---
역주
- js기준으로 설명하면 튜플은 '순서가 중요해서 사이즈 N으로 고정된 불변(immutable) 배열' 이라고 보면 됨.
    - 정확히는 서수(순서가 의미가 있는 내용)의 묶음(모음). 갯수N개에 대해서 N-tuple이라고 부르며 2-tuple을 흔히 쓰긴 함.(cf. 열거형)
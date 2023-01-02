# `if` expressions

`if`문은 다른 언어와 동일하게 사용할수 있습니다(조건의 ()는 없습니다):
> You use `if` very similarly to how you would in other languages:

```rust,editable
fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
}
```

게다가 `if`는 표현식(할당가능)으로 사용 될수도 있습니다.
아래 코드는 위와 동일한 결과 입니다.
> In addition, you can use it as an expression. This does the same as above:

```rust,editable
fn main() {
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
}
```

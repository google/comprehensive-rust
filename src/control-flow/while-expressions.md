# `while` expressions

`while` 역시 다른 언어와 동일한 사용법을 갖습니다:
> The `while` keyword works very similar to other languages:

```rust,editable
fn main() {
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
    }
    println!("Final x: {x}");
}
```


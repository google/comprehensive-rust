# `loop` expressions

마지막으로 `loop`키워드는 무한한 루프를 생성합니다. 따라서 반드시 `break` 또는 `return`를 사용해서 루프를 정지해야 합니다:
>  Finally, there is a `loop` keyword which creates an endless loop. Here you must
> either `break` or `return` to stop the loop:

```rust,editable
fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}
```

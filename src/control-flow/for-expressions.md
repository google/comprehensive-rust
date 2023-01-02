# `for` expressions

`for`표현식은 `while let` 표현식과 매우 유사합니다. `for`표현식은 자동으로 `into_iter()`를 호출한 다음 이를 반복합니다.
> The `for` expression is closely related to the `while let` expression. It will
> automatically call `into_iter()` on the expression and then iterate over it:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }
}
```

역시 다른언어와 마찬가지로 `break` 와 `continue`를 사용할 수 있습니다.
> You can use `break` and `continue` here as usual.

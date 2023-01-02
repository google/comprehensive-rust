# `Option` and `Result`

이 타입은 선택가능한 데이터를 표시합니다: 
> The types represent optional data:

```rust,editable
fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
}
```

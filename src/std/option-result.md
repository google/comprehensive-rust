# `Option` and `Result`

The types represent optional data:

```rust,editable
fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
}
```

<details>

* `Option` and `Result` are widely used not just in the standard library.
* `Option<&T>` has zero space overhead compared to `&T`.
* `Result` is the standard type to implement error handling as we will see on Day 3.
* `binary_search` returns `Result<usize, usize`.
  * If found, `Result::Ok` holds the index where the element is found.
  * Otherwise, `Result::Err` contains the index where such an element should be inserted.

</details>

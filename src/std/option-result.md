# `Option` and `Result`

The types represent optional data:

```rust,editable
fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let arr: Result<[i8; 3], Vec<i8>> = numbers.try_into();
    println!("arr: {arr:?}");
}
```

<details>

* `Option` and `Result` are widely used not just in the standard library.
* `Option<&T>` has zero space overhead compared to `&T`.
* `Result` is the standard type to implement error handling as we will see on Day 3.
* `try_into` attempts to convert the vector into a fixed-sized array. This can fail:
  * If the vector has the right size, `Result::Ok` is returned with the array.
  * Otherwise, `Result::Err` is returned with the original vector.

</details>

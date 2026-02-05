# Solution

```rust,editable
{{#include exercise.rs:types}}

{{#include exercise.rs:solution}}

{{#include exercise.rs:tests}}
```

- **`Result` Return Type:** The function signature changes to return
  `Result<i64, DivideByZeroError>`. This explicit type signature forces the
  caller to handle the possibility of failure.
- **The `?` Operator:** We use `?` on the recursive calls: `eval(*left)?`. This
  cleanly propagates errors. If `eval` returns `Err`, the function immediately
  returns that `Err`. If it returns `Ok(v)`, `v` is assigned to `left` (or
  `right`).
- **`Ok` Wrapping:** Successful results must be wrapped in `Ok(...)`.
- **Handling Division by Zero:** We explicitly check for `right == 0` and return
  `Err(DivideByZeroError)`. This replaces the panic in the original code.

<details>

- Mention that `DivideByZeroError` is a unit struct (no fields), which is
  sufficient here since there's no extra context to provide about the error.
- Discuss how `?` makes error handling almost as concise as exceptions, but with
  explicit control flow.

</details>

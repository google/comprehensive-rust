# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

<details>

- Note that the `P: Fn(u8, &str) -> bool` bound on the first `Filter` impl block
  isn't strictly necessary, but it helps with type inference when calling `new`.
  Demonstrate removing it and showing how the compiler now needs type
  annotations for the closure passed to `new`.

</details>

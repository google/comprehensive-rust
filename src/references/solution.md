# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

<details>

- Note that in `normalize` we were able to do `*item /= mag` to modify each
  element. This is because we're iterating using a reference to an array, which
  causes the `for` loop to give references to each element.

</details>

---
minutes: 20
---

# Exercise: Rewriting with Result

In this exercise we're revisiting the expression evaluator exercise that we did
in day 2. Our initial solution ignores a possible error case: Dividing by zero!
Rewrite `eval` to instead use idiomatic error handling to handle this error case
and return an error when it occurs. We provide a simple `DivideByZeroError` type
to use as the error type for `eval`.

```rust
{{#include exercise.rs:types}}

{{#include exercise.rs:eval}}

{{#include exercise.rs:tests}}
```

<details>

- The starting code here isn't exactly the same as the previous exercise's
  solution: We've added in an explicit panic to show students where the error
  case is. Point this out if students get confused.

</details>

---
minutes: 15
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Fibonacci

The Fibonacci sequence begins with `[0, 1]`. For `n > 1`, the next number is the
sum of the previous two.

Write a function `fib(n)` that calculates the nth Fibonacci number. When will
this function panic?

```rust,editable,should_panic
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:fib}}
    if n < 2 {
        // The base case.
        return todo!("Implement this");
    } else {
        // The recursive case.
        return todo!("Implement this");
    }
}

{{#include exercise.rs:main}}
```

<details>

- This exercise is a classic introduction to recursion.
- Encourage students to think about the base cases and the recursive step.
- The question "When will this function panic?" is a hint to think about integer
  overflow. The Fibonacci sequence grows quickly!
- Students might come up with an iterative solution as well, which is a great
  opportunity to discuss the trade-offs between recursion and iteration (e.g.,
  performance, stack overflow for deep recursion).

</details>

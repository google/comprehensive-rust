---
minutes: 15
---

# Exercise: Fibonacci

The Fibonacci sequence begins with `[0,1]`. For `n > 1`, the next number is the
sum of the previous two.

Write a function `fib(n)` that calculates the nth Fibonacci number. When will
this function panic?

```rust,editable,should_panic
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

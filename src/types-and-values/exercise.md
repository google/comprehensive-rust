---
minutes: 30
---

# Exercise: Fibonacci

The first and second Fibonacci numbers are both `1`. For n>2, the n'th Fibonacci
number is calculated recursively as the sum of the n-1'th and n-2'th Fibonacci
numbers.

Write a function `fib(n)` that calculates the n'th Fibonacci number. When will
this function panic?

```rust,editable,should_panic
{{#include exercise.rs:fib}}
    if n <= 2 {
        // The base case.
        todo!("Implement this")
    } else {
        // The recursive case.
        todo!("Implement this")
    }
}

{{#include exercise.rs:main}}
```

# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

We use the `return` syntax here to return values from the function. Later in the
course, we will see that the last expression in a block is automatically
returned, allowing us to omit the `return` keyword for a more concise style.

The `if` condition `n < 2` does not need parentheses, which is standard Rust
style.

## Panic

The exercise asks when this function will panic. The Fibonacci sequence grows
very rapidly. With `u32`, the calculated values will overflow the 32-bit integer
limit (4,294,967,295) when `n` reaches 48.

In Rust, integer arithmetic checks for overflow in _debug mode_ (which is the
default when using `cargo run`). If an overflow occurs, the program will panic
(crash with an error message). In _release mode_ (`cargo run --release`),
overflow checks are disabled by default, and the number will wrap around
(modular arithmetic), producing incorrect results.

<details>

- Walk through the solution step-by-step.
- Explain the recursive calls and how they lead to the final result.
- Discuss the integer overflow issue. With `u32`, the function will panic for
  `n` around 47. You can demonstrate this by changing the input to `main`.
- Show an iterative solution as an alternative and compare its performance and
  memory usage with the recursive one. An iterative solution will be much more
  efficient.

## More to Explore

For a more advanced discussion, you can introduce memoization or dynamic
programming to optimize the recursive Fibonacci calculation, although this is
beyond the scope of the current topic.

</details>

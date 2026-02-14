# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

The solution demonstrates the power of iterator method chaining to express
complex logic concisely:

- **Infinite Iteration:** `cycle()` creates an iterator that repeats the input
  indinitely, elegantly handling the wraparound requirement without modulo
  arithmetic.
- **Iterator Composition:** `zip()` combines the original and offset iterators.
  It naturally terminates when the shorter (finite) iterator is exhausted.
- **Functional Transformation:** `map()` applies the subtraction, and
  `collect()` performs the final transformation back into a `Vec`.

<details>

- **Laziness:** Iterator chains are lazy. No work is done until a terminal
  operation like `collect()` is called. The compiler is often able to optimize
  these chains into a single tight loop that is as efficient as imperative code.
- **Reference Handling:** Note that `values.iter()` yields references (`&i32`).
  In the `map` closure, we use the dereference operator (`*`) to access the
  underlying values for arithmetic.

</details>

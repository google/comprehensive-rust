---
minutes: 15
existing course material:
- exercises/day-1/morning.md
- exercises/day-1/afternoon.md
- exercises/day-2/morning.md
- exercises/day-2/afternoon.md
- exercises/day-3/morning.md
- exercises/day-3/afternoon.md
---

# Exercise: Collatz Sequence

The [Collatz Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) is
defined as follows, for an arbitrary n<sub>1</sub> greater than zero:

- If _n<sub>i</sub>_ is 1, then the sequence terminates at _n<sub>i</sub>_.
- If _n<sub>i</sub>_ is even, then _n<sub>i+1</sub> = n<sub>i</sub> / 2_.
- If _n<sub>i</sub>_ is odd, then _n<sub>i+1</sub> = 3 * n<sub>i</sub> + 1_.

Write a function to calculate the length of the collatz sequence for a given
initial `n`.

```rust,should_panic
{{#include exercise.rs:collatz_length}}
  todo!("Implement this")
}

{{#include exercise.rs:main}}
  todo!("Implement this")
}
```

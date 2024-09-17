---
minutes: 15
---

# Exercise: Collatz Sequence

The [Collatz Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) is
defined as follows, for an arbitrary n<sub>1</sub> greater than zero:

- If _n<sub>i</sub>_ is 1, then the sequence terminates at _n<sub>i</sub>_.
- If _n<sub>i</sub>_ is even, then _n<sub>i+1</sub> = n<sub>i</sub> / 2_.
- If _n<sub>i</sub>_ is odd, then _n<sub>i+1</sub> = 3 * n<sub>i</sub> + 1_.

For example, beginning with _n<sub>1</sub>_ = 3:

- 3 is odd, so _n<sub>2</sub>_ = 3 * 3 + 1 = 10;
- 10 is even, so _n<sub>3</sub>_ = 10 / 2 = 5;
- 5 is odd, so _n<sub>4</sub>_ = 3 * 5 + 1 = 16;
- 16 is even, so _n<sub>5</sub>_ = 16 / 2 = 8;
- 8 is even, so _n<sub>6</sub>_ = 8 / 2 = 4;
- 4 is even, so _n<sub>7</sub>_ = 4 / 2 = 2;
- 2 is even, so _n<sub>8</sub>_ = 1; and
- the sequence terminates.

Write a function to calculate the length of the collatz sequence for a given
initial `n`.

```rust,editable,should_panic
{{#include exercise.rs:collatz_length}}
  todo!("Implement this")
}

{{#include exercise.rs:tests}}

{{#include exercise.rs:main}}
```

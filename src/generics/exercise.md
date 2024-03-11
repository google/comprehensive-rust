---
minutes: 10
---

# Exercise: Generic `min`

In this short exercise, you will implement a generic `min` function that
determines the minimum of two values, using the [`Ord`] trait.

```rust,compile_fail
use std::cmp::Ordering;

// TODO: implement the `min` function used in `main`.

{{#include exercise.rs:main}}
```

<details>

- Show students the [`Ord`] trait and [`Ordering`] enum.

</details>

[`Ord`]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[`Ordering`]: https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html

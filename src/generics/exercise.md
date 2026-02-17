---
minutes: 10
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Generic `min`

In this short exercise, you will implement a generic `min` function that
determines the minimum of two values, using the [`Ord`] trait.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.

{{#include exercise.rs:tests}}
```

<details>

- Show students the [`Ord`] trait and [`Ordering`] enum.

</details>

[`Ord`]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[`Ordering`]: https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html

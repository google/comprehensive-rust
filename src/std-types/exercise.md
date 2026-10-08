---
minutes: 20
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Counter

In this exercise you will take a very simple data structure and make it generic.
It uses a
[`std::collections::HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)
to keep track of what values have been seen and how many times each one has
appeared.

The initial version of `Counter` is hardcoded to only work for `u32` values.
Make the struct and its methods generic over the type of value being tracked,
that way `Counter` can track any type of value.

If you finish early, try using the
[`entry`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry)
method to halve the number of hash lookups required to implement the `count`
method.

```rust,compile_fail,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::collections::HashMap;

/// Counter counts the number of times each value of type T has been seen.
struct Counter {
    values: HashMap<u32, u64>,
}

impl Counter {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given key.
    fn count(&mut self, key: u32) {
        if self.values.contains_key(&key) {
            *self.values.get_mut(&key).unwrap() += 1;
        } else {
            self.values.insert(key, 1);
        }
    }

    /// Return the number of times the given key has been seen.
    fn times_seen(&self, key: u32) -> u64 {
        self.values.get(&key).copied().unwrap_or_default()
    }
}

{{#include exercise.rs:main}}
```

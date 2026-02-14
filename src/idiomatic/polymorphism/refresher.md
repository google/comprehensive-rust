---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Refresher

Basic features of Rust's generics and polymorphism.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct HasGenerics<T>(...);

pub fn uses_traits<T: Debug>(input: T) {...}

pub trait TraitBounds: Clone {...}
```

<details>

- In this section we'll be going through the core concepts of Rust's approach to
  polymorphism, the things you'll run into the most in day-to-day usage.

</details>

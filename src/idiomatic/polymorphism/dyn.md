---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `dyn Trait` for Dynamic Dispatch in Rust

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait Trait {}

impl Trait for i32 {}
impl Trait for String {}

fn main() {
    let int: &dyn Trait = &42i32;
    let string: &dyn Trait = &String::from("Hello dyn!");;
}
```

<details>

- Our other main mechanism of doing dynamic polymorphism is `dyn`, which gives
  us dynamic dispatch through a trait interface.

- For any trait that is _dyn compatible_ we can coerce a reference to a value of
  that trait into a `dyn Trait` value.

- We call these _trait objects_. Their type is not known at compile time, but
  their behavior is: what is implemented by the trait itself.

- When you _need_ OOP-style heterogeneous data structures, you can reach for
  `Box<dyn Trait>`, but try to keep it homogeneous and generic-based first!

</details>

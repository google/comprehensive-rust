---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Polymorphism

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait Trait {}

pub struct HasGeneric<T>(T);

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

fn takes_generic<T: Trait>(value: &T) {}

fn takes_dyn(value: &dyn Trait) {}
```

<details>

- Rust has plenty of mechanisms for writing and using polymorphic code, but
  they're somewhat different from other popular languages!

- This chapter will cover the details of Rust's polymorphism and how it's
  similar, or different to, other languages.

</details>

---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Statically Sized and Dynamically Sized Types

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::fmt::Debug;

pub struct AlwaysSized<T /* : Sized */>(T);

pub struct OptionallySized<T: ?Sized>(T);

type Dyn1 = OptionallySized<dyn Debug>;
```

<details>

- Motivation: Being able to specify between types whose size are known and
  compile time and types whose size are known at runtime is useful for

- The Sized trait is automatically implemented by types with a known size at
  compile-time.

  This trait is also automatically added to any type parameter that doesn't
  opt-out of being sized.

- Most types implement `Sized`: they have a compile-time known size.

  Types like `[T]`, `str` and `dyn Trait` are all dynamically sized types. Their
  size is stored as part of the reference to the value of that type.

- Type parameters automatically implement `Sized` unless specified.

ref:

- https://doc.rust-lang.org/stable/reference/dynamically-sized-types.html#r-dynamic-sized

</details>

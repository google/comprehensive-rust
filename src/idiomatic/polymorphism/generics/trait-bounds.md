---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Trait Bounds on Generics

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::fmt::Display;

fn print_with_length<T: Display>(item: T) {
    println!("Item: {}", item);
    println!("Length: {}", item.to_string().len());
}

fn main() {
    print_with_length(42); // Works with integers
    print_with_length("Hello, Rust!"); // Works with strings
}
```

<details>

- Generic functions rely on traits to determine what operations are valid for a
  generic type.

- Without a trait bound on a generic type parameter, we don't have access to any
  behavior for that type. Remove the trait bound on `print_with_length` and
  demonstrate that we're not allowed to make assumptions about what methods it
  has.

ref:

- https://doc.rust-lang.org/reference/trait-bounds.html

</details>

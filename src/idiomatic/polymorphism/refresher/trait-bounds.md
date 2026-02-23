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
    let number = 42;
    let text = "Hello, Rust!";

    print_with_length(number); // Works with integers
    print_with_length(text); // Works with strings
}
```

<details>

- Traits are most commonly used as bounds on generic type parameters for a
  function or method.

  Without a trait bound on a generic type parameter, we don't have access to any
  behavior to write functions and methods with.

  Trait bounds allow us to specify the minimum viable behavior of a type for it
  to work in generic code.

ref:

- https://doc.rust-lang.org/reference/trait-bounds.html

</details>

---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Heterogeneous data with `dyn trait`

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::fmt::Display;

pub struct Lambda;

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "λ")
    }
}

fn main() {
    let heterogeneous: Vec<Box<dyn Display>> = vec![
        Box::new(42u32),
        Box::new(String::from("Woah")),
        Box::new(Lambda),
    ];
    for item in heterogeneous {
        // We know "item" implements Display, but we know nothing else!
        println!("Display output: {}", item);
    }
}
```

<details>

- `dyn Trait`, being a dynamic dispatch tool, lets us store heterogeneous data
  in collections.

- In this example, we're storing types that all implement `std::fmt::Display`
  and printing all items in that collection to screen.

</details>

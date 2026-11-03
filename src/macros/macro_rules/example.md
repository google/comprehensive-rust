---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# A Macros-By-Example Example

Let's look at a simple declarative macro that mimics the behavior of `vec![]`
for empty vector creation:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! my_vec {
    () => {
        Vec::new()
    };
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("Vector: {:?}", empty);
}
```

The macro is introduced with the `macro_rules!` construct, declaring a macro
named `my_vec`.

- **Match Arm:** The left-hand side `()` is the matcher. The pattern here
  matches only empty inputs.
- **Transcriber:** The right-hand side `{ Vec::new() }` is the transcriber. It
  specifies what the macro invocation will expand to for this pattern.

<details>

- Explain that `macro_rules!` is itself a macro, which uses a special, unique
  language syntax for defining declarative macros.
- Recall that although `my_vec![]` is invoked with square brackets (conventional
  for array-like objects), any brackets (`my_vec!()` or `my_vec!{}`) would also
  work.
- In this case, `my_vec![]` matches empty tokens and transcribes directly to
  `Vec::new()`.

</details>

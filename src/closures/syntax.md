---
minutes: 3
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Closure Syntax

Closures are created with vertical bars: `|..| ..`.

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    // Argument and return type can be inferred for lightweight syntax:
    let double_it = |n| n * 2;
    dbg!(double_it(50));

    // Or we can specify types and bracket the body to be fully explicit:
    let add_1f32 = |x: f32| -> f32 { x + 1.0 };
    dbg!(add_1f32(50.));
}
```

<details>

- The arguments go between the `|..|`. The body can be surrounded by `{ .. }`,
  but if it is a single expression these can be omitted.

- Argument types are optional, and are inferred if not given. The return type is
  also optional, but can only be written if using `{ .. }` around the body.

- The examples can both be written as mere nested functions instead -- they do
  not capture any variables from their lexical environment. We will see captures
  next.

## More to Explore

- The ability to store functions in variables doesn't just apply to closures,
  regular functions can be put in variables and then invoked the same way that
  closures can: [Example in the playground][fn-ptr].

  - The linked example also demonstrates that closures that don't capture
    anything can also coerce to a regular function pointer.

</details>

[fn-ptr]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=817cbeeefc49f3d0d180a3d6d54c8bda

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

- **Enums with Data:** Rust `enum` variants can carry data. `CarArrived(Floor)`
  carries an integer, and `ButtonPressed(Button)` carries a nested `Button`
  enum. This allows `Event` to represent a rich set of states in a type-safe
  way.
- **Type Aliases:** `type Floor = i32` gives a semantic name to `i32`. This
  improves readability, but `Floor` is still just an `i32` to the compiler.
- **`#[derive(Debug)]`:** We use this attribute to automatically generate code
  to format the enums for printing with `{:?}`. Without this, we would have to
  manually implement the `fmt::Debug` trait.
- **Nested Enums:** The `Button` enum is nested inside `Event::ButtonPressed`.
  This hierarchical structure is common in Rust for modeling complex domains.

<details>

- Note that `Event::CarDoorOpened` is a "unit variant" (it carries no data),
  while `Event::CarArrived` is a "tuple variant".
- You might discuss why `Button` is a separate enum rather than just having
  `LobbyCallButtonPressed` and `CarFloorButtonPressed` variants on `Event`. Both
  are valid, but grouping related concepts (like buttons) can make the code
  cleaner.

</details>

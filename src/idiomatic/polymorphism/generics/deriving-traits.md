---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Deriving Traits

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DrawingBuffer {
    target: [u8; 16],
    commands: Vec<String>,
}

// Many traits have simple implementations that can be generated automatically.
impl Clone for DrawingBuffer {
    fn clone(&self) -> Self {
        DrawingBuffer {
            target: self.target.clone(),
            commands: self.commands.clone(),
        }
    }
}
```

<details>

- Many traits have trivial implementations that would be easy to mechanically
  write. For these traits, it's possible to have the compiler generate the
  implementation for us using a derive macro.

- For example, the `Clone` trait is commonly implemented by simply cloning all
  fields of the struct. If this is the behavior we want for our type, we don't
  need to write out the boilerplate ourselves. Replace the manual implementation
  with the corresponding derive.

- Derived trait implementations automatically stay in sync with your type
  definition. Demonstrate adding a field to `DrawingBuffer` and show that the
  derived `Clone` impl is automatically updated, whereas the manual impl has to
  be updated by hand.

- Many standard library traits support being derived, and it's common for traits
  defined in the ecosystem to also support this.

- Traits do not support being derived by default, but you can implement derive
  support for your own traits with a macro. Doing so is covered in the Macros
  course.

references:

- https://doc.rust-lang.org/reference/attributes/derive.html#r-attributes.derive

</details>

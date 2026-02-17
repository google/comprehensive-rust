---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Common Traits to Implement

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone /* ... */)]
pub struct MyData {
    pub name: String,
    pub number: usize,
    pub data: [u8; 64],
}
```

<details>
- Traits are one of the most potent tools in the Rust language. The language and ecosystem expects you to use them, and so a big part of _predictability_ is what traits are implemented for a type!

- Traits should be liberally implemented on types you author, but there are
  caveats!

- Remember, many traits have the ability to be _derived_: to have a compiler
  plugin (macro) write the implementation for you!

- Authors of ecosystem traits (like De/Serialize) have made derive
  implementations for traits available to users, leading to very little
  commitment needed on the developer side for implementing these kinds of
  traits!

</details>

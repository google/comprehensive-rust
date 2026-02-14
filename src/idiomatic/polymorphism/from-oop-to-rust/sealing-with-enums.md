---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Sealing with Enums

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::collections::BTreeMap;
pub enum GetSource {
    WebUrl(String),
    BytesMap(BTreeMap<String, Vec<u8>>),
}

impl GetSource {
    fn get(&self, url: &str) -> Option<&Vec<u8>> {
        match self {
            Self::WebUrl(source) => unimplemented!(),
            Self::BytesMap(map) => map.get(url),
        }
    }
}
```

<details>

- Motivation: API is designed around a specific list of types that are valid for
  it, users of the API are not expected to extend it.

- Enums in Rust are _algebraic data types_, we can define different structures
  for each variant.

  For some domains, this might be enough polymorphism for the problem.
  Experiment and see what works, what solutions seem to make more sense.

- By having the user-facing part of the API refer to an enum, users know what
  types are valid inputs and can construct those types using the available
  methods to do so.

  - If the types that make up the enum have invariants that the API internally
    upholds, and the only way users can construct those types is through
    constructors that build and maintain those invariants, then you can be sure
    that inputs to a generic method uphold their invariants.

  - If the types that make up the enum instead are types the user can freely
    construct, then sanitisation and interpretation may need to be taken into
    consideration.

</details>

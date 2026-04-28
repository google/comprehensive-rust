---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Serialize/Deserialize style traits

Crates like `serde` can implement serialization automatically.

Derivable: ✅

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[derive(Serialize, Deserialize)]
struct ExtraData {
    fav_color: String,
    name_of_dog: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    age: usize,
    extra_data: ExtraData,
}
```

<details>

- Provides serialization and deserialization functionality for a type, allowing
  for Rust data types to be converted to/from data formats like JSON.

- The standard library doesn't have serialization functionality built-in, but
  the serde crate is the community standard interface for doing serialization.

- When not to implement: If a type contains sensitive data that should not be
  erroneously saved to disk or sent over a network, consider not implementing
  Serialize/Deserialize for that type.

  Shares security concerns with `Debug`, but given serialization is often used
  in networking there can be higher stakes.

</details>

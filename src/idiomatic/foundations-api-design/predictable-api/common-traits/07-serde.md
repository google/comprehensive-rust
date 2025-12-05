---
minutes: 5
---

Serialize/Deserialize style traits

Crates like `serde` can implement serialization automatically.

Derivable: ✅ When to implement: Almost always.

```rust,no_compile
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
- Provides serialization and deserialization functionality for a type.

- When not to implement: If a type contains sensitive data that should not be
  erroneously saved to disk or sent over a network, consider not implementing
  Serialize/Deserialize for that type.

  Shares security concerns with `Debug`, but given serialization is often used
  in networking there can be higher stakes.

</details>

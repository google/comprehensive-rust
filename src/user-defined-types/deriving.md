---
minutes: 5
existing course material:
- traits/deriving-traits.md
---

# Deriving

You can ask Rust to derive a number of useful behaviors for your custom types, as follows:

```rust,editable
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // Default trait adds `default` constructor.
    let mut p2 = p1.clone();    // Clone trait adds `clone` method.
    p2.name = String::from("EldurScrollz");
    // Debug trait adds support for printing with `{:?}`.
    println!("{:?} vs. {:?}", p1, p2);
}
```

<details>

These behaviors are "traits", which are covered later in the course. For now,
students only need to know that traits provide useful behavior, and that
`#[derive(..)]` is an easy way to add that behavior to types.

Derivation is implemented with macros, and many crates provide useful derive
macros to add useful functionality. For example, `serde` can derive
serialization support for a struct using `#[derive(Serialize)]`.

</detail>

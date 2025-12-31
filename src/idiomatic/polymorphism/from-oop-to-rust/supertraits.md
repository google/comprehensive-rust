---
minutes: 5
---

# "Inheritance" in Rust: Supertraits

```rust
pub trait SuperTrait {}

pub trait Trait: SuperTrait {}
```

<details>

- In Rust, traits can depend on other traits. We're already familiar with Traits
  being able to have Supertraits.

- This looks superficially similar to inheritance.

- This is a mechanism like inheritance, but separates the data from the
  behavior.

- Keeps behavior in a state where it's easy to reason about.

- Makes what we aim to achieve with "multiple inheritance" easier too:

  We only care about what behavior a type is capable of at the point where we
  clarify we want that behavior (when bounding a generic by traits).

  By specifying multiple traits on a generic, we know that the type has the
  methods of all those traits.

- Does not involve inheritance of fields. A trait doesn't expose fields, only
  methods and associated types / constants.

</details>

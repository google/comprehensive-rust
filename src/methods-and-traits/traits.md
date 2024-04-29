---
minutes: 15
---

# Traits

Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
trait Pet {
    /// Return a sentence from this pet.
    fn talk(&self) -> String;

    /// Print a string to the terminal greeting this pet.
    fn greet(&self);
}
```

<details>

- A trait defines a number of methods that types must have in order to implement
  the trait.

- In the "Generics" segment, next, we will see how to build functionality that
  is generic over all types implementing a trait.

</details>

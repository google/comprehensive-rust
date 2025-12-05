---
minutes: 10
---

# Deriving Traits

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BufferId([u8; 16]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DrawingBuffer {
    target: [u8; 16],
    commands: Vec<String>,
}
```

<details>
- Many traits, protocols, interfaces, have trivial implementations that would be easy to mechanically write.

- Definitions of types (their syntax trees) can be fed to procedural macros
  (compiler plugins) to automatically generate implementations of traits.

  These macros have to be authored by someone, the compiler cannot figure out
  everything by itself.

- Many traits have a naive, obvious implementation. Mostly implementations that
  depend on all fields or variants already implementing the trait.

  `PartialEq`/`Eq` can be derived on types whose fields / variants all implement
  those traits fairly easily: line up the fields / variants, if any of them
  don't match then the equality check returns false.

- Derives let us avoid boilerplate mechanically and predictably, the authors of
  a derive implementation likely authored the trait the derive was implemented
  with the proper semantics of a trait in mind.

- Ask the class: Have the students had to deal with a codebase where most of the
  code was trivial boilerplate?

- This is similar to Haskell's `deriving` system.

references:

- https://doc.rust-lang.org/reference/attributes/derive.html#r-attributes.derive

</details>

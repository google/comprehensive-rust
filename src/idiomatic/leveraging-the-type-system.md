---
minutes: 5
---

# Leveraging the Type System

Rust's type system is _expressive_: you can use types and traits to build
abstractions that make your code harder to misuse.

It is possible to enforce correctness at _compile-time_, with no runtime
overhead.

Types and traits can model concepts and constraints from your business domain.
With careful design, you can improve the clarity and maintainability of the
entire codebase.

<details>

Additional items speaker may mention:

- Rust's type system borrows ideas from functional programming languages.

  For example, Rust's enums are known as "algebraic data types" in languages
  like Haskell and OCaml. You can take inspiration from learning material geared
  towards functional languages when looking for guidance on how to design with
  types. ["Domain Modeling Made Functional"][1] is a great resource on the
  topic, with examples written in F#.

- Despite Rust's functional roots, not all functional design patterns can be
  easily translated to Rust.

  For example, you must have a solid grasp on a broad selection of advanced
  topics to design APIs that leverage higher-order functions and higher-kinded
  types in Rust.

  Evaluate, on a case-by-case basis, whether a more imperative approach may be
  easier to implement. Consider using in-place mutation, relying on Rust's
  borrow-checker and type system to control what can be mutated, and where.

- The same caution should be applied to object-oriented design patterns. Rust
  doesn't support inheritance, and object decomposition should take into account
  the constraints introduced by the borrow checker.

- Mention that type-level programming creates "zero-cost abstractions", although
  the label can be misleading: the impact on compile times and code complexity
  may be significant.

</details>

{{%segment outline}}

[1]: https://pragprog.com/titles/swdddf/domain-modeling-made-functional/

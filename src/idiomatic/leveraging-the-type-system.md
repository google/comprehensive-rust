---
minutes: 5
---

# Leveraging the Type System

Rust's type system is _expressive_: you can use types and traits to build
abstractions that make your code harder to misuse.

In some cases, you can go as far as enforcing correctness at _compile-time_,
with no runtime overhead.

Types and traits can model concepts and constraints from your business domain.
With careful design, you can improve the clarity and maintainability of the
entire codebase.

<details>

Additional items speaker may mention:

- Rust's type system borrows a lot of ideas from functional programming
  languages.\
  For example, Rust's enums are known as "algebraic data types" in languages
  like Haskell and OCaml. You can take inspiration from learning material geared
  towards functional languages when looking for guidance on how to design with
  types. ["Domain Modeling Made Functional"][1] is a great resource on the
  topic, with examples written in F#.

- Despite Rust's functional roots, functional design patterns don't translate as-is
  to Rust. For instance, extensive use of higher-order functions and higher-kinded types can
  result in code that is harder to read and maintain. Design patterns in Rust
  must take into account (and leverage!) the granular control over mutability
  that comes with its borrow checker.

- The same caution should be applied to object-oriented design patterns. Rust
  doesn't support inheritance, and object decomposition should take into account the
  constraints introduced by the borrow checker.

- Mention that type-level abstractions are often referred to as "zero-cost
  abstractions", although the label can be misleading: the impact on compile
  times and code complexity may be significant.

</details>

{{%segment outline}}

[1]: https://pragprog.com/titles/swdddf/domain-modeling-made-functional/

---
minutes: 5
---

# Leveraging the Type System

Rust's type system is _expressive_.\
We can use types and traits to build abstractions that make our code harder to
misuse. In some cases, we can even go as far as enforcing correctness at
_compile-time_. Quite often, these abstractions have no runtime
overhead[^zero-cost].

The type system can also be used to model concepts and constraints from your
business domain. By designing our types carefully, we can improve the clarity
and maintainability of the entire codebase.

<details>

Additional items speaker may mention:

- Rust's type system borrows a lot of ideas from functional programming
  languages.\
  For example, Rust's enums are known as "algebraic data types" in languages
  like Haskell and OCaml. You can take inspiration from learning material geared
  towards functional languages when looking for guidance on how to design with
  types. ["Domain Modeling Made Functional"][1] is a great resource on the
  topic, with examples written in F#.

- Despite its functional roots, functional design patterns don't translate as-is
  to Rust. For instance, extensive use of higher-kinded functions and types can
  result in code that is harder to read and maintain. Design patterns in Rust
  must take into account (and leverage!) the granular control over mutability
  that comes with its borrow-checker.

- The same caution should be applied to object-oriented design patterns. Rust
  doesn't support inheritance, and object boundaries must be mindful of the
  constraints introduced by the borrow-checker.

</details>

{{%segment outline}}

[1]: https://pragprog.com/titles/swdddf/domain-modeling-made-functional/

[^zero-cost]: They often referred to as "zero-cost abstractions", although the
    label can be misleading: the impact on compile times and code complexity may
    be significant.

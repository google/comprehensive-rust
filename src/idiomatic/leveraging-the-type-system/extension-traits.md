---
minutes: 15
---

# Extension Traits

It may desirable to **extend** foreign types with new inherent methods. For
example, allow your code to check if a string is a palindrome using
method-calling syntax: `s.is_palindrome()`.

It might feel natural to reach out for an `impl` block:

```rust,editable,compile_fail
// 🛠️❌
impl str {
    pub fn is_palindrome(&self) -> bool {
        self.chars().eq(self.chars().rev())
    }
}
```

The Rust compiler won't allow it, though. But you can use the **extension trait
pattern** to work around this limitation.

<details>

- A Rust item (be it a trait or a type) is referred to as:

  - **foreign**, if it isn't defined in the current crate
  - **local**, if it is defined in the current crate

  The distinction has significant implications for
  [coherence and orphan rules][1], as we'll get a chance to explore in this
  section of the course.

- Compile the example to show the compiler error that's emitted.

  Highlight how the compiler error message nudges you towards the extension
  trait pattern.

- Explain how the numerous type-system restrictions in Rust aim to prevent
  _ambiguity_.

  What would happen if you were allowed to define new inherent methods on
  foreign types? Different crates in your dependency tree might end up defining
  different methods on the same foreign type with the same name.

  As soon as there is room for ambiguity, there must be a way to disambiguate.
  If disambiguation happens implicitly, it can lead to surprising or otherwise
  unexpected behavior. If disambiguation happens explicitly, it can increase the
  cognitive load on developers who are reading your code.

  Furthermore, every time a crate defines a new inherent method on a foreign
  type, it may cause compilation errors in _your_ code, as you may be forced to
  introduce explicit disambiguation.

  Rust has decided to avoid the issue altogether by forbidding the definition of
  new inherent methods on foreign types.

- Other languages (e.g, Kotlin, C#, Swift) allow adding methods to existing
  types, known as "extension methods." This leads to different trade-offs in
  terms of potential ambiguities and the need for global reasoning.

</details>

[1]: https://doc.rust-lang.org/stable/reference/items/implementations.html#r-items.impl.trait.orphan-rule

---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Should I Define An Extension Trait?

In what scenarios should you prefer an extension trait over a free function?

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait StrExt {
    fn is_palindrome(&self) -> bool;
}

impl StrExt for &str {
    fn is_palindrome(&self) -> bool {
        self.chars().eq(self.chars().rev())
    }
}

// vs

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
```

The main advantage of extension traits is **ease of discovery**.

<details>

- Extension methods can be easier to discover than free functions. Language
  servers (e.g., `rust-analyzer`) will suggest them if you type `.` after an
  instance of the foreign type.

- However, a bespoke extension trait might be overkill for a single method. Both
  approaches require an additional import, and the familiar method syntax may
  not justify the boilerplate of a full trait definition.

- **Discoverability:** Extension methods are easier to discover than free
  functions. Language servers (e.g., `rust-analyzer`) will suggest them if you
  type `.` after an instance of the foreign type.

- **Method Chaining:** A major ergonomic win for extension traits is method
  chaining. This is the foundation of the `Iterator` trait, allowing for fluent
  calls like `data.iter().filter(...).map(...)`. Achieving this with free
  functions would be far more cumbersome (`map(filter(iter(data), ...), ...)`).

- **API Cohesion:** Extension traits help create a cohesive API. If you have
  several related functions for a foreign type (e.g., `is_palindrome`,
  `word_count`, `to_kebab_case`), grouping them in a single `StrExt` trait is
  often cleaner than having multiple free functions for a user to import.

- **Trade-offs:** Despite these advantages, a bespoke extension trait might be
  overkill for a single, simple function. Both approaches require an additional
  import, and the familiar method syntax may not justify the boilerplate of a
  full trait definition.

</details>

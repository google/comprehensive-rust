---
minutes: 5
---

# Should I Define An Extension Trait?

In what scenarios should you prefer an extension trait over a free function?

```rust
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

- A bespoke extension trait might be an overkill if you want to add a single
  method to a foreign type. Both a free function and an extension trait will
  require an additional import, and the familiarity of the method calling syntax
  may not be enough to justify the boilerplate of a trait definition.

  Nonetheless, extension methods can be **easier to discover** than free
  functions. In particular, language servers (e.g. `rust-analyzer`) will suggest
  extension methods if you type `.` after an instance of the foreign type.

</details>

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

- Extension methods can be easier to discover than free functions. Language servers (e.g., `rust-analyzer`) will suggest them if you type `.` after an instance of the foreign type.

- However, a bespoke extension trait might be overkill for a single method. Both approaches require an additional import, and the familiar method syntax may not justify the boilerplate of a full trait definition.

</details>

---
minutes: 5
---

# Display

"Write to string" trait, prioritizing readability for an end user.

Derivable: ❌, without crates like `derive_more`.

When to implement: As-needed, for errors and other types that an end-user will
see.

```rust
#[derive(Debug)]
pub enum NetworkError {
    HttpCode(u16),
    WhaleBitTheUnderseaCable,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::HttpCode(code) => write!(f, "HTTP Error code {code}"),
            NetworkError::WhaleBitTheUnderseaCable => {
                write!(f, "Whale attack detected – call Ishmael")
            }
        }
    }
}

impl std::error::Error for NetworkError {}
```

<details>
- A trait similar to `Debug`, but with a focus on end-user readability.

- Prerequisite for the `Error` trait.

  If implementing for an error type, focus on providing a descriptive error for
  users and programmers other than you.

- Same security considerations as Debug, consider the ways that sensitive data
  could be exposed in UI or logs.

- Types that implement `Display` automatically have `ToString` implemented for
  them.

</details>

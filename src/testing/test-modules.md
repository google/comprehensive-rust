---
minutes: 5
existing course material:
- testing.md
- testing/test-modules.md
- testing/unit-tests.md
---

# Test Modules

# Testing

Rust and Cargo come with a simple unit test framework:

* Unit tests are supported throughout your code.

* Integration tests are supported via the `tests/` directory.
# Test Modules

Unit tests are often put in a nested module (run tests on the
[Playground](https://play.rust-lang.org/)):

```rust,editable
fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}
```

* This lets you unit test private helpers.
* The `#[cfg(test)]` attribute is only active when you run `cargo test`.
# Unit Tests

Mark unit tests with `#[test]`:

```rust,editable,ignore
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}
```

Use `cargo test` to find and run the unit tests.

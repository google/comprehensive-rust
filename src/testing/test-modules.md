# Test Modules

Unit tests are often put in a nested module. If you run the code below, tests
are not executed, so you’ll need to run tests on the
[Playground](https://play.rust-lang.org/).

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

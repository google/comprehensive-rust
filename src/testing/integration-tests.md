# Integration Tests

If you want to test your library as a client, use an integration test.

Create a `.rs` file under `tests/`:

```rust,ignore
use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```

These tests only have access to the public API of your crate.

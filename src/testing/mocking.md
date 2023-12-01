---
minutes: 5
---

# Mocking

If you want to do mocking, we recommend [Mockall]. You need to refactor your
code to use traits, which you can then quickly mock:

```rust,ignore
{{#include mockall.rs}}
```

[Mockall]: https://docs.rs/mockall/

<details>

- Note that *mocking is controversial*: mocks allow you to completely isolate a
  test from its dependencies. The immediate result is faster and more stable
  test execution. On the other hand, the mocks can be configured wrongly and
  return output different from what the real dependencies would do.

  If at all possible, it is recommended that you use the real dependencies. As
  an example, many databases allow you to configure an in-memory backend. This
  means that you get the correct behavior in your tests, plus they are fast and
  will automatically clean up after themselves.

  Similarly, many web frameworks allow you to start an in-process server which
  binds to a random port on `localhost`. Always prefer this over mocking away
  the framework since it helps you test your code in the real environment.

- Mockall is not part of the Rust Playground, so you need to run this example in
  a local environment. Use `cargo add mockall` to quickly add Mockall to an
  existing Cargo project.

- Mockall has a lot more functionality. In particular, you can set up
  expectations which depend on the arguments passed:

    ```rust,ignore
    let mut mock_cat = MockPet::new();
    mock_cat
        .expect_is_hungry()
        .with(mockall::predicate::gt(Duration::from_secs(3 * 3600)))
        .return_const(true);
    mock_cat
        .expect_is_hungry()
        .return_const(true);
    assert_eq!(mock_cat.is_hungry(Duration::from_secs(1 * 3600)), false);
    assert_eq!(mock_cat.is_hungry(Duration::from_secs(5 * 3600)), true);
    ```

- You can use `.times(n)` to limit the number of times a mock method can be
  called to `n` --- the mock will automatically panic when dropped if this isn't
  satisfied.

- Mockall is available for use in AOSP.

</details>

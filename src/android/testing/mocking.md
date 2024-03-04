---
minutes: 5
---

# Mocking

For mocking, [Mockall] is a widely used library. You need to refactor your code
to use traits, which you can then quickly mock:

```rust,ignore
{{#include mockall.rs:simple_example}}
```

[Mockall]: https://docs.rs/mockall/

<details>

- Mockall is the recommended mocking library in Android (AOSP). There are other
  [mocking libraries available on crates.io](https://crates.io/keywords/mock),
  in particular in the area of mocking HTTP services. The other mocking
  libraries work in a similar fashion as Mockall, meaning that they make it easy
  to get a mock implementation of a given trait.

- Note that mocking is somewhat _controversial_: mocks allow you to completely
  isolate a test from its dependencies. The immediate result is faster and more
  stable test execution. On the other hand, the mocks can be configured wrongly
  and return output different from what the real dependencies would do.

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
  expectations which depend on the arguments passed. Here we use this to mock a
  cat which becomes hungry 3 hours after the last time it was fed:

```rust,ignore
{{#include mockall.rs:extended_example}}
```

- You can use `.times(n)` to limit the number of times a mock method can be
  called to `n` --- the mock will automatically panic when dropped if this isn't
  satisfied.

</details>

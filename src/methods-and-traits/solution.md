# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Trait Implementation:** We implement the `Logger` trait for
  `VerbosityFilter`, matching the signature defined in the trait.
- **Composition and Delegation:** The `VerbosityFilter` struct wraps another
  logger (the `inner` field). In its implementation of `log`, it adds logic (the
  verbosity check) and then delegates the actual logging to the inner logger if
  appropriate.
- **Wrapper Pattern:** This demonstrates how to extend behavior by wrapping a
  type. In this specific case, `VerbosityFilter` is hardcoded to wrap
  `StderrLogger`, but in real-world Rust code, `inner` would typically be
  generic over any type that implements `Logger`.

<details>

- Note that `self.inner.log(...)` works because `StderrLogger` also implements
  `Logger`.
- Point out that this wrapper adds functionality (filtering) without modifying
  the inner logger's code. This is a key benefit of using traits and
  composition.

</details>

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

The solution demonstrates how traits enable polymorphic behavior through
composition:

- **Trait Implementation:** `VerbosityFilter` implements the `Logger` trait,
  allowing it to be used anywhere a `Logger` is expected.
- **Wrapper Pattern:** The `VerbosityFilter` struct wraps an instance of
  `StderrLogger`. It intercepts the `log` call to apply filtering logic before
  delegating to the inner logger.
- **Composition over Inheritance:** Behavior is extended by wrapping one type in
  another, rather than through a class hierarchy.

<details>

- **Extensibility:** This wrapper pattern (often called a decorator or
  middleware) allows adding functionality like filtering, buffering, or
  timestamping without modifying the underlying logger.
- **Teaser for Generics:** While `VerbosityFilter` is hardcoded to wrap
  `StderrLogger`, it could be made generic to wrap _any_ type that implements
  `Logger`. This is covered in the next section.

</details>

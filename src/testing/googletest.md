---
minutes: 5
---

# GoogleTest

The [GoogleTest](https://docs.rs/googletest/) crate allows for flexible test
assertions using _matchers_:

```rust,ignore
{{#include googletest.rs:test_elements_are}}
```

If we change the last element to `"!"`, the test fails with a structured error
message pin-pointing the error:

<!-- mdbook-xgettext: skip -->

```text
---- test_elements_are stdout ----
Value of: value
Expected: has elements:
  0. is equal to "foo"
  1. is less than "xyz"
  2. starts with prefix "!"
Actual: ["foo", "bar", "baz"],
  where element #2 is "baz", which does not start with "!"
  at src/testing/googletest.rs:6:5
Error: See failure output above
```

<details>

- GoogleTest is not part of the Rust Playground, so you need to run this example
  in a local environment. Use `cargo add googletest` to quickly add it to an
  existing Cargo project.

- The `use googletest::prelude::*;` line imports a number of
  [commonly used macros and types][prelude].

- This just scratches the surface, there are many builtin matchers.

- A particularly nice feature is that mismatches in multi-line strings strings
  are shown as a diff:

```rust,ignore
{{#include googletest.rs:test_multiline_string_diff}}
```

shows a color-coded diff (colors not shown here):

<!-- mdbook-xgettext: skip -->

```text
    Value of: haiku
Expected: is equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
Actual: "Memory safety found,\nRust's strong typing guides the way,\nSecure code you'll write.",
  which isn't equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
Difference(-actual / +expected):
 Memory safety found,
-Rust's strong typing guides the way,
+Rust's silly humor guides the way,
 Secure code you'll write.
  at src/testing/googletest.rs:17:5
```

- The crate is a Rust port of
  [GoogleTest for C++](https://google.github.io/googletest/).

[prelude]: https://docs.rs/googletest/latest/googletest/prelude/index.html

- GoogleTest is available for use in AOSP.

</details>

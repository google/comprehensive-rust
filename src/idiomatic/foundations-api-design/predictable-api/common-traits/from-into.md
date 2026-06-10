---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# From & Into

Conversion from one type to another.

Derivable: ❌, without crates like `derive_more`.

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct Wrapper(String);

impl From<&str> for Wrapper {
    fn from(value: &str) -> Self {
        Wrapper(value.to_owned())
    }
}

impl From<i32> for Wrapper {
    fn from(value: i32) -> Self {
        Wrapper(value.to_string())
    }
}

// `Into` is more natural to use as a trait bound.
fn into_string<S: Into<String>>(s: S) {}
fn string_from<T>(t: T) where String: From<T> {}

fn main() {
    // `Wrapper` can be construct from `&str` and `i32`.
    let a = Wrapper::from("Hello, obvious!");
    let b = Wrapper::from(-123);

    // A From implementation implies an Into implementation.
    let c: Wrapper = "Hello, implementation!".into();
}
```

<details>

- Provides conversion functionality to types.

- `From` provides a constructor-style function, whereas `Into` provides a method
  on an existing value.

- Prefer writing `From<T>` implementations for a type you're authoring instead
  of `Into<T>`.

  The `Into` trait is implemented for any type that implements `From`
  automatically.

- `Into` is preferred as a trait bound for arguments to functions for clarity of
  intent for what the function can take: `T: Into<String>` has clearer intent
  than `String: From<T>`.

</details>

# Error Type Compatibility

When defining a custom error type, there are some common conventions to follow
in order to be compatible with the broader ecosystem:

- Implement the standard library's [`Error`] trait.
- Derive `Debug` and implement `Display` (required by `Error`).
- Implement `From` conversions for underlying error types to support automatic
  error type conversion with `?`.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::error::Error;
use std::io::Read;
use std::{fmt, fs, io};

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername,
}

impl Error for ReadUsernameError {}

impl fmt::Display for ReadUsernameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {e}"),
            Self::EmptyUsername => write!(f, "Username file was empty"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
```

<details>

- Any type can be used as the error type for `Result`: There are no trait
  requirements, structs and enums are both valid, and we can even use
  empty/zero-sized types like `()` when the error doesn't need to cary any data.

- While there aren't any strict requirements for error types, there are a couple
  of conventions to follow for ecosystem compatibility.

- We've already seen how the `From` trait can be used to support automatic error
  type conversion with the `?` operator.

- The standard library provides the `Error` marker trait for indicating that a
  type represents an error.

- `Error` depends on `Debug` (which can be derived) and `Display` (which must be
  implemented manually). These ensure that an error can always be turned into a
  string for the purpose of logging or displaying the error to a user.

- The `Error` trait doesn't provide any functionality of its own, but in a
  couple of slides we will look at how `Error` can be used to create dynamic
  error types.

- These trait implementations represent a bit of boilerplate that needs to be
  written for each custom error type. On the next slide we'll look at a crate in
  the ecosystem that can be used to cut down on this boilerplate.

</details>

[`Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html

---
minutes: 5
---

# `thiserror`

The [`thiserror`](https://docs.rs/thiserror/) crate provides macros to help
avoid boilerplate when defining error types. It provides derive macros that
assist in implementing `From<T>`, `Display`, and the `Error` trait.

```rust,editable,compile_fail
use std::fs;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
}
```

<details>

- The `Error` derive macro is provided by `thiserror`, and has lots of useful
  attributes to help define error types in a compact way.
- The message from `#[error]` is used to derive the `Display` trait.
- Note that the (`thiserror::`)`Error` derive macro, while it has the effect of
  implementing the (`std::error::`)`Error` trait, is not the same this; traits
  and macros do not share a namespace.

</details>

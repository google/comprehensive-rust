---
minutes: 5
existing course material:
- error-handling/error-contexts.md
- error-handling/dynamic-errors.md
---

<!-- NOTES:
Quick demo of using `thiserror` and `anyhow` to handle errors, including adding context
-->
# thiserror and anyhow

# Adding Context to Errors

The widely used [anyhow](https://docs.rs/anyhow/) crate can help you add
contextual information to your errors and allows you to have fewer
custom error types:

```rust,editable,compile_fail
use std::{fs, io};
use std::io::Read;
use anyhow::{Context, Result, bail};

fn read_username(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!("Found no username in {path}");
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err:?}"),
    }
}
```

<details>

* `anyhow::Result<V>` is a type alias for `Result<V, anyhow::Error>`.
* `anyhow::Error` is essentially a wrapper around `Box<dyn Error>`. As such it's again generally not
  a good choice for the public API of a library, but is widely used in applications.
* Actual error type inside of it can be extracted for examination if necessary.
* Functionality provided by `anyhow::Result<T>` may be familiar to Go developers, as it provides
  similar usage patterns and ergonomics to `(T, error)` from Go.

</details>
# Dynamic Error Types

Sometimes we want to allow any type of error to be returned without writing our own enum covering
all the different possibilities. `std::error::Error` makes this easy.

```rust,editable,compile_fail
use std::fs;
use std::io::Read;
use thiserror::Error;
use std::error::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(EmptyUsernameError(String::from(path)).into());
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}
```

<details>

This saves on code, but gives up the ability to cleanly handle different error cases differently in
the program. As such it's generally not a good idea to use `Box<dyn Error>` in the public API of a
library, but it can be a good option in a program where you just want to display the error message
somewhere.

</details>

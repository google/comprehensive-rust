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

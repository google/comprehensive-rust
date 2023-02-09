# Deriving Error Enums

The [thiserror](https://docs.rs/thiserror/) crate is a popular way to create an
error enum like we did on the previous page:

```rust,editable,compile_fail
use std::{fs, io};
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("Could not read: {0}")]
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
        Err(err)     => println!("Error: {err}"),
    }
}
```

<details>

`thiserror`'s derive macro automatically implements `std::error::Error`, and optionally `Display`
(if the `#[error(...)]` attributes are provided) and `From` (if the `#[from]` attribute is added).
It also works for structs.

It doesn't affect your public API, which makes it good for libraries.

</details>

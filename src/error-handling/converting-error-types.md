# Converting Error Types

The effective expansion of `?` is a little more complicated than previously indicated:

```rust,ignore
expression?
```

works the same as

```rust,ignore
match expression {
    Ok(value) => value,
    Err(err)  => return Err(From::from(err)),
}
```

The `From::from` call here means we attempt to convert the error type to the
type returned by the function:

```rust,editable
use std::{fs, io};
use std::io::Read;

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
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
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
```

<details>

Key points:

* The `username` variable can be either `Ok(string)` or `Err(error)`.
* Use the `fs::write` call to test out the different scenarios: no file, empty file, file with username.

It is good practice for all error types to implement `std::error::Error`, which requires `Debug` and
`Display`. It's generally helpful for them to implement `Clone` and `Eq` too where possible, to make
life easier for tests and consumers of your library. In this case we can't easily do so, because
`io::Error` doesn't implement them.

</details>

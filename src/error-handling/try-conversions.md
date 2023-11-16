---
minutes: 5
---

# Try Conversions

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
type returned by the function. This makes it easy to encapsulate errors into
higher-level errors.

## Example

```rust,editable
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
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

The return type of the function has to be compatible with the nested functions it calls. For instance,
a function returning a `Result<T, Err>` can only apply the `?` operator on a function returning a
`Result<AnyT, Err>`. It cannot apply the `?` operator on a function returning an `Option<AnyT>` or `Result<T, OtherErr>`
unless `OtherErr` implements `From<Err>`. Reciprocally, a function returning an `Option<T>` can only apply the `?` operator
on a function returning an `Option<AnyT>`.

You can convert incompatible types into one another with the different `Option` and `Result` methods
such as `Option::ok_or`, `Result::ok`, `Result::err`.


It is good practice for all error types that don't need to be `no_std` to implement `std::error::Error`, which requires `Debug` and `Display`. The `Error` crate for `core` is only available in [nightly](https://github.com/rust-lang/rust/issues/103765), so not fully `no_std` compatible yet.

It's generally helpful for them to implement `Clone` and `Eq` too where possible, to make
life easier for tests and consumers of your library. In this case we can't easily do so, because
`io::Error` doesn't implement them.

A common alternative to a `From` implementation is `Result::map_err`, especially when the conversion only happens in one place.

</details>

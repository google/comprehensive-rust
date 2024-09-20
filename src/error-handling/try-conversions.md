---
minutes: 5
---

# Try Conversions

The effective expansion of `?` is a little more complicated than previously
indicated:

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
use std::fs::File;
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
            Self::IoError(e) => write!(f, "I/O error: {e}"),
            Self::EmptyUsername(path) => write!(f, "Found no username in {path}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
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
    //std::fs::write("config.dat", "").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
```

<details>

The `?` operator must return a value compatible with the return type of the
function. For `Result`, it means that the error types have to be compatible. A
function that returns `Result<T, ErrorOuter>` can only use `?` on a value of
type `Result<U, ErrorInner>` if `ErrorOuter` and `ErrorInner` are the same type
or if `ErrorOuter` implements `From<ErrorInner>`.

A common alternative to a `From` implementation is `Result::map_err`, especially
when the conversion only happens in one place.

There is no compatibility requirement for `Option`. A function returning
`Option<T>` can use the `?` operator on `Option<U>` for arbitrary `T` and `U`
types.

A function that returns `Result` cannot use `?` on `Option` and vice versa.
However, `Option::ok_or` converts `Option` to `Result` whereas `Result::ok`
turns `Result` into `Option`.

</details>

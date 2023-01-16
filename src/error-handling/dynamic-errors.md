# Dynamic Error Types

Sometimes we want to allow any type of error to be returned without writing our own enum covering
all the different possibilities. `std::error::Error` makes this easy.

```rust,editable,compile_fail
use std::fs::{self, File};
use std::io::Read;
use thiserror::Error;
use std::error::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
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

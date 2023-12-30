---
minutes: 5
---

# Try Operator

Runtime errors like connection-refused or file-not-found are handled with the
`Result` type, but matching this type on every call can be cumbersome. The
try-operator `?` is used to return errors to the caller. It lets you turn the
common

```rust,ignore
match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

into the much simpler

```rust,ignore
some_expression?
```

We can use this to simplify our error handling code:

```rust,editable
use std::io::Read;
use std::{fs, io};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
```

<details>

Simplify the `read_username` function to use `?`.

Key points:

- The `username` variable can be either `Ok(string)` or `Err(error)`.
- Use the `fs::write` call to test out the different scenarios: no file, empty
  file, file with username.
- Note that `main` can return a `Result<(), E>` as long as it implements
  `std::process:Termination`. In practice, this means that `E` implements
  `Debug`. The executable will print the `Err` variant and return a nonzero exit
  status on error.

</details>

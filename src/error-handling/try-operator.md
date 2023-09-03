# Propagating Errors with `?`

The try-operator `?` is used to return errors to the caller. It lets you turn
the common

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
use std::{fs, io};
use std::io::Read;

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

Key points:

* The `username` variable can be either `Ok(string)` or `Err(error)`.
* Use the `fs::write` call to test out the different scenarios: no file, empty file, file with username.
* The return type of the function has to be compatible with the nested functions it calls. For instance,
a function returning a `Result<T, Err>` can only apply the `?` operator on a function returning a 
`Result<AnyT, Err>`. It cannot apply the `?` operator on a function returning an `Option<AnyT>` or `Result<T, OtherErr>`
unless `OtherErr` implements `From<Err>`. Reciprocally, a function returning an `Option<T>` can only apply the `?` operator 
on a function returning an `Option<AnyT>`.
    * You can convert incompatible types into one another with the different `Option` and `Result` methods 
    such as `Option::ok_or`, `Result::ok`, `Result::err`.

</details>

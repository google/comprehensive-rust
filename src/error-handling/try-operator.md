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

We can use this to simplify our error handing code:

```rust,editable
use std::fs;
use std::io::{self, Error, ErrorKind, Read};

// Reads the email address from a file and returns the username and hostname.
fn read_email_address(path: &str) -> Result<(String, String), io::Error> {
    let email = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    if let Some((username, hostname)) = email.split_once('@') {
        Ok((String::from(username), String::from(hostname)))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Invalid email address"))
    }
}


fn main() {
    //fs::write("config.dat", "alice@gmail.com").unwrap();
    let email = read_email_address("config.dat");
    println!("email or error: {email:?}");
}
```

<details>

Key points:

* The `username` variable can be either `Ok(string)` or `Err(error)`.
* Use the `fs::write` call to test out the different scenarios: no file, empty file, file with username.
* The return type of the function has to be compatible with the nested functions it calls. For instance,
a function returning a `Result<T, Err>` can only apply the `?` operator on a function returning a 
`Result<AnyT, Err>`. It cannot apply the `?` operator on a function returning a `Result<T, OtherErr>` 
or an `Option<AnyT>`. Reciprocally, a function returning an `Option<T>` can only apply the `?` operator 
on a function returning an `Option<AnyT>`.
    * You can convert incompatible types into one another with the different `Option` and `Result` methods 
    such as `Option::ok_or`, `Result::ok`, `Result::err`.

</details>

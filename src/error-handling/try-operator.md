# Propagating Errors with `?`

The try-operator `?` is used to return errors to the caller. It lets you turn
the common

```rust,ignore
let some_variable = match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
};
```

into the much simpler

```rust,ignore
let some_variable = some_expression?;
```

We can use this to simplify our error handing code:

```rust,editable
use std::fs;
use std::io::{self, Read};

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

It also works with `Option<T>`

```rust,editable
fn add_or_none(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    let a = if let Some(a) = a {
        a
    } else {
        return None;
    };

    let b = if let Some(b) = b {
        b
    } else {
        return None;
    };

    Some(a + b)
}

fn main() {
    let a = Some(1);
    let b = Some(2);
    println!("None + None = {:?}", add_or_none(None, None));
    println!("a + None = {:?}", add_or_none(a, None));
    println!("None + b = {:?}", add_or_none(None, b));
    println!("a + b = {:?}", add_or_none(a, b));
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
    * You can coerce incompatible types into one another with the different `Option` and `Result` methods 
    such as `Option::ok_or`, `Result::ok`, `Result::err`.
    * Error types can be implicitely converted into the return type (let's call it`TopLevelErrorT`) with the `?` operator if they implement the `Into<TopLevelErrorT>` trait. This can be a good exercise to 
    try out.

</details>

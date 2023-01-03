# Propagating Errors with `?`

시도(시행)연산자 `?`는 호출자에게 오류를 반환할 때 사용합니다. 
> The try-operator `?` is used to return errors to the caller. It lets you turn
> the common

```rust,ignore
match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

훨씬 간단한 방식으로
> into the much simpler

```rust,ignore
some_expression?
```

이를 사용하면 오류를 처리할 수 단순화 할 수 있습니다:
> We can use this to simplify our error handing code:

```rust,editable
use std::fs;
use std::io::{self, Read};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username: {username:?}");
}
```

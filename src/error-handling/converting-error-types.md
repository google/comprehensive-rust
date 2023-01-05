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

`From::from` 호출은 오류타입을 함수에서 반환하는 타입으로 변환합니다: 
> The `From::from` call here means we attempt to convert the error type to the
> type returned by the function:

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
    println!("username: {username:?}");
}
```

# Deriving Error Enums

[thiserror](https://docs.rs/thiserror/) 크레이트는 전페이지에서 처럼 
오류 enum을 만드는 일반적인 방법입니다.
> The [thiserror](https://docs.rs/thiserror/) crate is a popular way to create an
> error enum like we did on the previous page:

```rust,editable,compile_fail
use std::{fs, io};
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
enum ReadUsernameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
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
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}
```

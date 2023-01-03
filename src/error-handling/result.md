# Structured Error Handling with `Result`

`Result` enum은 흔히 오류를 예상되는 경우 사용됩니다: 
> We have already see the `Result` enum. This is used pervasively when errors are
> expected as part of normal operation:

```rust
use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        },
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}
```

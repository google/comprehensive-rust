# Structured Error Handling with `Result`

We have already seen the `Result` enum. This is used pervasively when errors are
expected as part of normal operation:

```rust,editable
use std::fs;
use std::io::Read;

fn main() {
    let file = fs::File::open("diary.txt");
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

<details>

  * As with `Option`, the successful value sits inside of `Result`, forcing the developer to
    explicitly extract it. This encourages error checking. In the case where an error should never happen,
    `unwrap()` or `expect()` can be called, and this is a signal of the developer intent too.  
  * `Result` documentation is a recommended read. Not during the course, but it is worth mentioning. 
    It contains a lot of convenience methods and functions that help functional-style programming. 
    
</details>

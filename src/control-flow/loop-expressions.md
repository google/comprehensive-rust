# `loop` expressions

Finally, there is a `loop` keyword which creates an endless loop. Here you must
either `break` or `return` to stop the loop:

```rust,editable
fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}
```

<details>
    
* Break the `loop` with a value (e.g. `break 8`) and print it out.

</details>

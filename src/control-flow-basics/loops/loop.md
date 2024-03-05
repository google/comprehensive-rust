# `loop`

The [`loop` statement](https://doc.rust-lang.org/std/keyword.loop.html) just
loops forever, until a `break`.

```rust,editable
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
```

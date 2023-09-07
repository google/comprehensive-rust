---
minutes: 2
---

# Macros

The Rust standard library includes an assortment of useful macros. Macros are distinguished by a `!` at the end.

* `println!(format, ..)` prints a line to standard output, applying formatting described in [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html).
* `format!(format, ..)` works just like `println!` but returns the result as a string.
* `dbg!(expression)` logs the value of the expression and returns it.
* `todo!()` marks a bit of code as not-yet-implemented. If executed, it will panic.
* `unreachable!()` marks a bit of code as unreachable. If executed, it will panic.

```rust,editable
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

fn main() {
    let n = 13;
    println!("{n}! = {}", factorial(4));
}
```

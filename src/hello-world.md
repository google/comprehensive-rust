# Hello World!

Let us jump into the simplest possible Rust program, a classic Hello World
program:

```rust
fn main() {
    println!("Hello 🌍!");
}
```

What you see:

* Functions are introduced with `fn`.
* Blocks are delimited by curly braces like in C and C++.
* The `main` function is the entry point of the program.
* Rust has hygienic macros, `println!` is an example of this.
* Rust strings are UTF-8 encoded and can contain any Unicode character.

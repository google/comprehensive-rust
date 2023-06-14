# Small Example

Here is a small example program in Rust:

```rust,editable
fn main() {              // Program entry point
    let mut x: i32 = 6;  // Mutable variable binding
    print!("{x}");       // Macro for printing, like printf
    while x != 1 {       // No parenthesis around expression
        if x % 2 == 0 {  // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
```

<details>

The code implements the Collatz conjecture: it is believed that the loop will
always end, but this is not yet proved. Edit the code and play with different
inputs.

Key points:

* Explain that all variables are statically typed. Try removing `i32` to trigger
  type inference. Try with `i8` instead and trigger a runtime integer overflow.

* Change `let mut x` to `let x`, discuss the compiler error.

* Show how `print!` gives a compilation error if the arguments don't match the
  format string.

* Show how you need to use `{}` as a placeholder if you want to print an
  expression which is more complex than just a single variable.

* Show the students the standard library, show them how to search for `std::fmt`
  which has the rules of the formatting mini-language. It's important that the
  students become familiar with searching in the standard library.
    
    * In a shell `rustup doc std::fmt` will open a browser on the local std::fmt documentation

</details>

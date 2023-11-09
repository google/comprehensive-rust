---
minutes: 5
existing course material:
- hello-world.md
- hello-world/small-example.md
---

# Hello, World

# Hello World!

Let us jump into the simplest possible Rust program, a classic Hello World
program:

```rust,editable
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

<details>

This slide tries to make the students comfortable with Rust code. They will see
a ton of it over the next three days so we start small with something familiar.

Key points:

* Rust is very much like other languages in the C/C++/Java tradition. It is
  imperative and it doesn't try to reinvent things unless
  absolutely necessary.

* Rust is modern with full support for things like Unicode.

* Rust uses macros for situations where you want to have a variable number of
  arguments (no function [overloading](basic-syntax/functions-interlude.md)).

* Macros being 'hygienic' means they don't accidentally capture identifiers from
  the scope they are used in. Rust macros are actually only
  [partially hygienic](https://veykril.github.io/tlborm/decl-macros/minutiae/hygiene.html).

* Rust is multi-paradigm. For example, it has powerful [object-oriented programming features](https://doc.rust-lang.org/book/ch17-00-oop.html),
  and, while it is not a functional language, it includes a range of [functional concepts](https://doc.rust-lang.org/book/ch13-00-functional-features.html).

</details>
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

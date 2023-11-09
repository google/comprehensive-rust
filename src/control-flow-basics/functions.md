---
minutes: 3
existing course material:
- basic-syntax/functions.md
- basic-syntax/rustdoc.md
- basic-syntax/methods.md
- basic-syntax/functions-interlude.md
---

<!-- NOTES:
Simple functions and the return statement
-->
# Functions

# Functions

A Rust version of the famous [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) interview question:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    print_fizzbuzz_to(20);
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}
```

<details>

* We refer in `main` to a function written below. Neither forward declarations nor headers are necessary.
* Declaration parameters are followed by a type (the reverse of some programming languages), then a return type.
* The last expression in a function body (or any block) becomes the return value. Simply omit the `;` at the end of the expression.
* Some functions have no return value, and return the 'unit type', `()`. The compiler will infer this if the `-> ()` return type is omitted.
* The range expression in the `for` loop in `print_fizzbuzz_to()` contains `=n`, which causes it to include the upper bound.

</details>
# Rustdoc

All language items in Rust can be documented using special `///` syntax.

```rust,editable
/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
///
/// # Example
/// ```
/// assert!(is_divisible_by(42, 2));
/// ```
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}
```

The contents are treated as Markdown. All published Rust library crates are
automatically documented at [`docs.rs`](https://docs.rs) using the
[rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) tool. It is
idiomatic to document all public items in an API using this pattern.
Code snippets can document usage and will be used as unit tests.

<details>

* Show students the generated docs for the `rand` crate at
  [`docs.rs/rand`](https://docs.rs/rand).

* This course does not include rustdoc on slides, just to save space, but in
  real code they should be present.

* Inner doc comments are discussed later (in the page on modules) and need not
  be addressed here.

* Rustdoc comments can contain code snippets that we can run and test using `cargo test`.
  We will discuss these tests in the [Testing section](../testing/doc-tests.html).

</details>
# Methods

Methods are functions associated with a type. The `self` argument of a method is
an instance of the type it is associated with:

```rust,editable
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
```

* We will look much more at methods in today's exercise and in tomorrow's class.

<details>

- Add a static method called `Rectangle::new` and call this from `main`:

    ```rust,editable,compile_fail
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    ```

- While _technically_, Rust does not have custom constructors, static methods are commonly used to initialize structs (but don't have to).
  The actual constructor, `Rectangle { width, height }`, could be called directly. See the [Rustnomicon](https://doc.rust-lang.org/nomicon/constructors.html).

- Add a `Rectangle::square(width: u32)` constructor to illustrate that such static methods can take arbitrary parameters.

</details>
# Function Overloading

Overloading is not supported:

* Each function has a single implementation:
  * Always takes a fixed number of parameters.
  * Always takes a single set of parameter types.
* Default values are not supported:
  * All call sites have the same number of arguments.
  * Macros are sometimes used as an alternative.

However, function parameters can be generic:

```rust,editable
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}
```

<details>

* When using generics, the standard library's `Into<T>` can provide a kind of limited
  polymorphism on argument types. We will see more details in a later section.

</details>


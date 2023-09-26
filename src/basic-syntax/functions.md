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

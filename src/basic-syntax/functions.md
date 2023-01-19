# Functions

A Rust version of the famous [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) interview question:

```rust,editable
fn main() {
    fizzbuzz_to(20);   // Defined below, no forward declaration needed
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression is the return value
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true,  true)  => println!("fizzbuzz"),
        (true,  false) => println!("fizz"),
        (false, true)  => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
    for n in 1..=n {
        fizzbuzz(n);
    }
}
```

<details>

* We refer in `main` to a function written below. Neither forward declarations nor headers are necessary. 
* Declaration parameters are followed by a type (the reverse of some programming languages), then a return type.
* The last expression in a function body becomes the return value. Simply omit the `;` at the end of the expression.
* Some functions have no return value, and output the 'unit type', `()`. The compiler will infer this if the `-> ()` return type is omitted.

</details>

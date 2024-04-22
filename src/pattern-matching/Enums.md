# Enums

Like tuples, enums can also be destructured by matching:

Patterns can also be used to bind variables to parts of your values. This is how
you inspect the structure of your types. Let us start with a simple `enum` type:

```rust,editable
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
```

Here we have used the arms to _destructure_ the `Result` value. In the first
arm, `half` is bound to the value inside the `Ok` variant. In the second arm,
`msg` is bound to the error message.

<details>

- The `if`/`else` expression is returning an enum that is later unpacked with a
  `match`.
- You can try adding a third variant to the enum definition and displaying the
  errors when running the code. Point out the places where your code is now
  inexhaustive and how the compiler tries to give you hints.
- The values in the enum variants can only be accessed after being pattern
  matched.
- Demonstrate what happens when the search is inexhaustive. Note the advantage
  the Rust compiler provides by confirming when all cases are handled.
- Save the result of `divide_in_two` in the `result` variable and `match` it in
  a loop. That won't compile because `msg` is consumed when matched. To fix it,
  match `&result` instead of `result`. That will make `msg` a reference so it
  won't be consumed. This
  ["match ergonomics"](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html)
  appeared in Rust 2018. If you want to support older Rust, replace `msg` with
  `ref msg` in the pattern.

</details>

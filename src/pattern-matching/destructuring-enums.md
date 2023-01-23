# Destructuring Enums

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
        Result::Err(format!("cannot divide {} into two equal parts", n))
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

Consider demonstrating what happens if you add the third enum variant. Rust
will try to be helpful and the code won't compile, because you haven't handled
the new case in your match statements.

</details>

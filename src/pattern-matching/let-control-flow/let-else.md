# `let else` Statements

For the common case of matching a pattern and returning from the function, use
[`let else`](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html).
The "else" case must diverge (`return`, `break`, or panic - anything but falling
off the end of the block).

```rust,editable
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = match maybe_string {
        Some(s) => s,
        None => return Err(String::from("got None")),
    };

    let first_byte_char = match s.chars().next() {
        Some(first) => first,
        None => return Err(String::from("got empty string")),
    };

    match first_byte_char.to_digit(16) {
        Some(digit) => Ok(digit),
        None => Err(String::from("not a hex digit")),
    }
}

fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
```

<details>
The rewritten version is:

```rust
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}
```

## More to Explore

- This early return-based control flow is common in Rust error handling code,
  where you try to get a value out of a `Result`, returning an error if the
  `Result` was `Err`.
- If students ask, you can also demonstrate how real error handling code would
  be written with `?`.

</details>

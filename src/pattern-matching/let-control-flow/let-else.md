# `let else` Statements

For the common case of matching a pattern and returning from the function, use
[`let else`](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html).
The "else" case must diverge (`return`, `break`, or panic - anything but falling
off the end of the block).

```rust,editable
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    // TODO: The structure of this code is difficult to follow -- rewrite it with let-else!
    if let Some(s) = maybe_string {
        if let Some(first_byte_char) = s.chars().next() {
            if let Some(digit) = first_byte_char.to_digit(16) {
                Ok(digit)
            } else {
                return Err(String::from("not a hex digit"));
            }
        } else {
            return Err(String::from("got empty string"));
        }
    } else {
        return Err(String::from("got None"));
    }
}

fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
```

<details>

`if-let`s can pile up, as shown. The `let-else` construct supports flattening
this nested code. Rewrite the awkward version for students, so they can see the
transformation.

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

    return Ok(digit);
}
```

</details>

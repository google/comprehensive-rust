---
minutes: 10
---

# Let Control Flow

Rust has a few control flow constructs which differ from other languages. They
are used for pattern matching:

- `if let` expressions
- `while let` expressions
- `match` expressions

# `if let` expressions

The
[`if let` expression](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)
lets you execute different code depending on whether a value matches a pattern:

```rust,editable
fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };
    std::thread::sleep(dur);
    println!("slept for {:?}", dur);
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
```

# `let else` expressions

For the common case of matching a pattern and returning from the function, use
[`let else`](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html).
The "else" case must diverge (`return`, `break`, or panic - anything but falling
off the end of the block).

```rust,editable
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("not a hex digit"))
    }
}

fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
```

Like with `if let`, there is a
[`while let`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops)
variant which repeatedly tests a value against a pattern:

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
    // (There are more efficient ways to reverse a string!)
}
```

Here
[`String::pop`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.pop)
returns `Some(c)` until the string is empty, after which it will return `None`.
The `while let` lets us keep iterating through all items.

<details>

## if-let

- Unlike `match`, `if let` does not have to cover all branches. This can make it
  more concise than `match`.
- A common usage is handling `Some` values when working with `Option`.
- Unlike `match`, `if let` does not support guard clauses for pattern matching.

## let-else

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

# while-let

- Point out that the `while let` loop will keep going as long as the value
  matches the pattern.
- You could rewrite the `while let` loop as an infinite loop with an if
  statement that breaks when there is no value to unwrap for `name.pop()`. The
  `while let` provides syntactic sugar for the above scenario.

</details>

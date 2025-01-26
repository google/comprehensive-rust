---
minutes: 10
---

# Matching Values

The `match` keyword lets you match a value against one or more _patterns_. The
patterns can be simple values, similarly to `switch` in C and C++, but they can
also be used to express more complex conditions:

```rust,editable
#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }
}
```

A variable in the pattern (`key` in this example) will create a binding that can
be used within the match arm. We will learn more about this on the next slide.

A match guard causes the arm to match only if the condition is true. If the
condition is false the match will continue checking later cases.

<details>

Key Points:

- You might point out how some specific characters are being used when in a
  pattern
  - `|` as an `or`
  - `..` can expand as much as it needs to be
  - `1..=5` represents an inclusive range
  - `_` is a wild card

- Match guards as a separate syntax feature are important and necessary when we
  wish to concisely express more complex ideas than patterns alone would allow.
- They are not the same as separate `if` expression inside of the match arm. An
  `if` expression inside of the branch block (after `=>`) happens after the
  match arm is selected. Failing the `if` condition inside of that block won't
  result in other arms of the original `match` expression being considered.
- The condition defined in the guard applies to every expression in a pattern
  with an `|`.

# More To Explore

- Another piece of pattern syntax you can show students is the `@` syntax which
  binds a part of a pattern to a variable. For example:

  ```rust
  let opt = Some(123);
  match opt {
      outer @ Some(inner) => {
          println!("outer: {outer:?}, inner: {inner}");
      }
      None => {}
  }
  ```

  In this example `inner` has the value 123 which it pulled from the `Option`
  via destructuring, `outer` captures the entire `Some(inner)` expression, so it
  contains the full `Option::Some(123)`. This is rarely used but can be useful
  in more complex patterns.

</details>

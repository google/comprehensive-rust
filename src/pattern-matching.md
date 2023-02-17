# Pattern Matching

The `match` keyword let you match a value against one or more _patterns_. The
comparisons are done from top to bottom and the first match wins.

The patterns can be simple values, similarly to `switch` in C and C++:

```rust,editable
fn main() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}
```

The `_` pattern is a wildcard pattern which matches any value.

<details>
    
Key Points:
* You might point out how some specific characters are being used when in a pattern
  * `|` as an `or`
  * `..` can expand as much as it needs to be
  * `1..=5` represents an inclusive range
  * `_` is a wild card
* It can be useful to show how binding works, by for instance replacing a wildcard character with a variable, or removing the quotes around `q`.
* You can demonstrate matching on a reference.
* This might be a good time to bring up the concept of irrefutable patterns, as the term can show up in error messages.
   
</details>

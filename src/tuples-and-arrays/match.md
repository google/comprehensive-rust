---
minutes: 10
existing course material:
- control-flow/match-expressions.md
- pattern-matching.md
- pattern-matching/match-guards.md
---

<!-- NOTES:
Match as statement / expression, matching on ranges, guard expressions
-->
# Pattern Matching

# `match` expressions

The [`match` keyword](https://doc.rust-lang.org/reference/expressions/match-expr.html)
is used to match a value against one or more patterns. In that sense, it works
like a series of `if let` expressions:

```rust,editable
fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}
```

Like `if let`, each match arm must have the same type. The type is the last
expression of the block, if any. In the example above, the type is `()`.

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

* Save the match expression to a variable and print it out.
* Remove `.as_deref()` and explain the error.
    * `std::env::args().next()` returns an `Option<String>`, but we cannot match against `String`.
    * `as_deref()` transforms an `Option<T>` to `Option<&T::Target>`. In our case, this turns `Option<String>` into `Option<&str>`.
    * We can now use pattern matching to match against the `&str` inside `Option`.
</details>
# Pattern Matching

The `match` keyword lets you match a value against one or more _patterns_. The
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
# Match Guards

When matching, you can add a _guard_ to a pattern. This is an arbitrary Boolean
expression which will be executed if the pattern matches:

```rust,editable
{{#include ../../third_party/rust-by-example/match-guards.rs}}
```

<details>

Key Points:
* Match guards as a separate syntax feature are important and necessary when we wish to concisely express more complex ideas than patterns alone would allow.
* They are not the same as separate `if` expression inside of the match arm. An `if` expression inside of the branch block (after `=>`) happens after the match arm is selected. Failing the `if` condition inside of that block won't result in other arms
of the original `match` expression being considered.
* You can use the variables defined in the pattern in your if expression.
* The condition defined in the guard applies to every expression in a pattern with an `|`.
</details>

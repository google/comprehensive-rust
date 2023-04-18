# `if let` expressions

The [`if let`
expression](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)
lets you execute different code depending on whether a value matches a pattern:

```rust,editable
fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}
```

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

* `if let` can be more concise than `match`, e.g., when only one case is interesting. In contrast, `match` requires all branches to be covered.
* A common usage is handling `Some` values when working with `Option`.
* Unlike `match`, `if let` does not support guard clauses for pattern matching.
* Since 1.65, a similar [let-else](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html) construct allows to do a destructuring assignment, or if it fails, have a non-returning block branch (panic/return/break/continue):

   ```rust,editable
   fn main() {
       println!("{:?}", second_word_to_upper("foo bar"));
   }
    
   fn second_word_to_upper(s: &str) -> Option<String> {
       let mut it = s.split(' ');
       let (Some(_), Some(item)) = (it.next(), it.next()) else {
           return None;
       };
       Some(item.to_uppercase())
   }

</details>

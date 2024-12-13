---
minutes: 5
---

# `match` Expressions

`match` can be used to check a value against one or more options:

```rust,editable
fn main() {
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
}
```

Like `if` expressions, `match` can also return a value;

```rust,editable
fn main() {
    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {val}");
}
```

<details>

- `match` arms are evaluated from top to bottom, and the first one that matches
  has its corresponding body executed.

- There is no fall-through between cases the way that `switch` works in other
  languages.

- The body of a `match` arm can be a single expression or a block. Technically
  this is the same thing, since blocks are also expressions, but students may
  not fully understand that symmetry at this point.

- `match` expressions need to be exhaustive, meaning they either need to cover
  all possible values or they need to have a default case such as `_`.
  Exhaustiveness is easiest to demonstate with enums, but enums haven't been
  introduced yet. Instead we demonstrate matching on a `bool`, which is the
  simplest primitive type.

- This slide introduces `match` without talking about pattern matching, giving
  students a chance to get familiar with the syntax without front-loading too
  much information. We'll be talking about pattern matching in more detail
  tomorrow, so try not to go into too much detail here.

## More to Explore

- To further motivate the usage of `match`, you can compare the examples to
  their equivalents written with `if`. In the second case matching on a `bool`
  an `if {} else {}` block is pretty similar. But in the first example that
  checks multiple cases, a `match` expression can be more concise than
  `if {} else if {} else if {} else`.

- `match` also supports match guards, which allow you to add an arbitrary
  logical condition that will get evaluated to determine if the match arm should
  be taken. However talking about match guards requires explaining about pattern
  matching, which we're trying to avoid on this slide.

</details>

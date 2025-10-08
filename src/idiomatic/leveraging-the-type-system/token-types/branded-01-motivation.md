---
minutes: 10
---

# Variable-Specific Tokens (Branding 1/4)

What if we want to tie a token to a specific variable?

```rust,editable
struct Data {}
struct VariableSpecificToken {}

impl Data {
    fn check_permission(&self) -> Option<VariableSpecificToken> {
        Some(VariableSpecificToken {})
    }
    fn use_token(&self, token: &VariableSpecificToken) {}
}

fn main() {
    let data_1 = Data {};
    if let Some(token_1) = data_1.check_permission() {
        data_1.use_token(&token_1); // Works fine!
        let data_2 = Data {};
        data_2.use_token(&token_1); // But how do we prevent this?
    }
}
```

<details>

- What if we want to tie a token to a _specific variable_ in our code? Can we do
  this in Rust's type system?

- Motivation: Say we want a "proof of index" for a variable of some type, but we
  don't want that index to "cross over" to variables of the same or different
  types.

  Or what if we want to model "arena allocation" somehow?

- Ask: How might we try to do this?

  Expect students to not reach a good implementation from this, but be willing
  to experiment and follow through on suggestions.

- This kind of token-association is called Branding. Doing this lets us expand
  the "proof of work from elsewhere" to more general aspects of rust.

- [`GhostCell`](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf) is a
  prominent user of this, later slides will touch on it.

</details>

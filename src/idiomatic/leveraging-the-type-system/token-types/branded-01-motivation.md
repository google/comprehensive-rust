---
minutes: 10
---

# Value Specific Tokens (Branding 1/4)

What if we want there to be a token tied to a specific value?

```rust,editable
struct Data {}
struct ValueSpecificToken {}

impl Data {
    fn get_token(&self) -> ValueSpecificToken {
        ValueSpecificToken {}
    }
    fn use_token(&self, token: &ValueSpecificToken) {}
}

fn main() {
    let branded_1 = Data {};
    let token_1 = branded_1.get_token();
    branded_1.use_token(&token_1); // Works fine!
    let branded_2 = Data {};
    let token_2 = branded_2.get_token();

    // But how do we prevent this?
    branded_1.use_token(&token_2);
    branded_2.use_token(&token_1);
}
```

<details>

- What if we want to tie a token to a _specific variable_ in our code? Can we do
  this in Rust's type system?

- Motivation: Say we want "proof of index" for a value of some type, but we
  don't want that index to "cross over" to values of the same or different
  types.

  Or what if we want to model "arena allocation" somehow?

- Ask: How might we try to do this?

  Expect students to not reach a good implementation from this, but be willing
  to experiment and follow through on things.

- This kind of token-association is called Branding. Doing this lets us expand
  the "proof of work from elsewhere" to more general aspects of rust.

- [`GhostCell`](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf) is a
  prominent user of this, later slides will touch on it.

</details>

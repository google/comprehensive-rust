---
minutes: 15
---

# Token Types

Types with private constructors can be used to act as proof of invariants.

<!-- dprint-ignore-start -->
```rust,editable
pub mod token {
    // A public type with private fields behind a module boundary.
    pub struct Token { proof: () }

    pub fn get_token() -> Option<Token> {
        Some(Token { proof: () })
    }
}

pub fn protected_work(token: token::Token) {
    println!("We have a token, so we can make assumptions.")
}

fn main() {
    if let Some(token) = token::get_token() {
        // We have a token, so we can do this work.
        protected_work(token);
    } else {
        // We could not get a token, so we can't call `protected_work`.
    }
}
```
<!-- dprint-ignore-end -->

<details>

- Motivation: We want to be able to restrict user's access to functionality
  until they've performed a specific task.

  We can do this by defining a type the API consumer cannot construct on their
  own, through privacy tools of structs and modules.

- Ask: What is the purpose of the `proof: ()` field here?

  Without `proof: ()`, `Token` would have no private fields and users would be
  able to construct values of `Token` arbitrarily.

- By putting the `Token` type behind the module `token`, users outside that
  module can't construct the value on their own as they don't have permission to
  access the `proof` field.

  The API developer gets to define methods and functions that produce these
  tokens. The user does not.

  The token becomes a proof that one has met the API developer's conditions of
  access for those tokens.

- Ask: How might an API developer accidentally introduce ways to circumvent
  this?

  Expect answers like "serialization implementations" or other parser
  implementations. Or an implementation of `Default`.

</details>

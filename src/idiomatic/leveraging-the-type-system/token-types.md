---
minutes: 0
---

# Token Types

```rust,editable
pub mod token {
    pub struct Token {
        proof: ()
    }

    pub fn get_token() -> Option<Token> {
        Some(Token { proof: () })
    }
}

pub fn work_that_requires_a_token(token: Token) {
    // Do important work that has prerequisites
    println!("Function was given a token! We can make assumptions here while we work.")
}

fn main() {
    if let Some(token) = token::get_token() {
        // We have a token, so we can do this work.
        work_that_requires_a_token(token);
    } else {
        // We could not get a token, so we couldn't call the function that
        // needs a value of this type.
        println!("Could not get token.");
    }
}
```

<details>

- Token types let us use the privacy tools of types and modules to control when an API consumer has access to a value of a certain type, and have it be a requirement that a consumer has a value of that type in order to perform certain actions.

  A user of the API cannot construct a value of a certain type on their own, so they need to go through "proper" channels to do so. The designer of the API gets to choose those proper channels and how they behave.

- There's some overlap between this and general API design, because in rust you cannot pass "null" values for any type. If a function asks for a value, you have to construct that value! If you can't construct that value yourself, you need to call functions that can!

- By an API user showing they have access to a value of a certain type, we can assume that whatever invariants we put around the construction of that type likely apply. 

</details>
---
minutes: 0
---

# Token Types

```rust,editable
pub mod token {
    pub struct Token {
        proof: ()
    };

    pub fn prepare_environment() -> Option<Token> {
        Some(Token { proof: () })
    }

}

pub fn do_work_that_needed_prep(token: Token) {
    // Do important work that has prerequisites
    println!("Token shown, gated work done!")
}

fn main() {
    if let Some(token) = token::prepare_environment() {
        do_work_that_needed_prep(token)
    } else {
        println!("Could not get token.")
    }
}
```

<details>

- Token types let us use the privacy tools of types and modules to control when an API consumer has access to a value of a certain type, and have it be a requirement that a consumer has a value of that type in order to perform certain actions.

- This enforces "I have permission to do X" at the type level.

</details>
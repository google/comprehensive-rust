---
minutes: 5
---

# Permission Tokens

Token types work well as a trivial "proof of checked permission."

```rust,editable
mod admin {
    pub struct AdminToken(());

    pub fn get_admin(password: &str) -> Option<AdminToken> {
        if password == "Password123" { Some(AdminToken(())) } else { None }
    }
}

// We don't have to check that we have permissions, because
// the AdminToken argument is equivalent to such a check.
pub fn add_moderator(_: &admin::AdminToken, user: &str) {}

fn main() {
    if let Some(token) = admin::get_admin("Password123") {
        add_moderator(&token, "CoolUser");
    } else {
        eprintln!("Incorrect password! Could not prove privileges.")
    }
}
```

<details>

- This example shows modelling gaining administrator privileges for a chat
  client with a password and giving a user a moderator rank once those
  privileges are gained. The `AdminToken` type acts as "proof of correct user
  privileges."

  The user asked for a password in-code and if we get the password correct, we
  get a `AdminToken` to perform administrator actions within a specific
  environment (here, a chat client).

  Once the permissions are gained, we can call a `add_moderator` function.

  We can't call that function without the token type, so by being able to call
  it at all all we can assume we have permissions.

</details>

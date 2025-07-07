---
minutes: 5
---

# Semantic Confusion

When a function takes multiple arguments of the same type, call sites are
unclear:

```rust
# struct LoginError;
pub fn login(username: &str, password: &str) -> Result<(), LoginError> {
    // [...]
    # Ok(())
}

# let password = "password";
# let username = "username";
// In another part of the codebase, we swap arguments by mistake.
// Bug (best case), security vulnerability (worst case)
login(password, username);
```

The newtype pattern can prevent this class of errors at compile time:

```rust
pub struct Username(String);
pub struct Password(String);
# struct LoginError;

pub fn login(username: &Username, password: &Password) -> Result<(), LoginError> {
    // [...]
    # Ok(())
}

# let password = Password("password".into());
# let username = Username("username".into());
login(password, username); // 🛠️❌
```

<details>

- Run both examples to show students the successful compilation for the original
  example, and the compiler error returned by the modified example.

- Stress the _semantic_ angle. The newtype pattern should be leveraged to use
  distinct types for distinct concepts, thus ruling out this class of errors
  entirely.

- Nonetheless, note that there are legitimate scenarios where a function may
  take multiple arguments of the same type. In those scenarios, if correctness
  is of paramount importance, consider using a struct with named fields as
  input:
  ```rust
  pub struct LoginArguments {
      pub username: &str,
      pub password: &str,
  }
  # fn login(i: LoginArguments) {}
  # let password = "password";
  # let username = "username";

  // No need to check the definition of the `login` function to spot the issue.
  login(LoginArguments {
      username: password,
      password: username,
  })
  ```
  Users are forced, at the callsite, to assign values to each field, thus
  increasing the likelihood of spotting bugs.

</details>

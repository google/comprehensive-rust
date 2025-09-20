---
minutes: 0
---

# Permission Tokens

```rust,editable
mod sudo {
    // A public type with private fields kept behind a module
    // to enforce privacy boundaries.
    pub struct SuToken(())

    pub fn get_sudo(password: &str) -> Option<SuToken> {
        if password == "Password123" {
            Some(SuToken(()))
        } else {
            None
        }
    }
}

// We don't have to check that we have permissions, because
// the SuToken argument is equivalent to such a check.
pub fn install_packages(_: &SuToken, packages: &[&str]) {
    for package in packages {
        // Not doing any real package work here.
        println!("Retrieving package {package}...");
    }
    for package in packages {
        println!("Installed package {package}!");
    }
}

fn main() {
    if let Some(token) = get_sudo("Password123") {
        install_packages(&token, &[
            "golang",
            "google-chrome-stable",
            "texlive-full",
        ]);
    } else {
        eprintln!("Incorrect password! Could not prove privileges.")
    }
}
```

<details>

- This example shows modelling gaining root privileges with a password and installing packages to the system once those privileges are gained. 
  
  Here, we're asked for a password in-code and if we get the password correct, we get a `SuToken` to perform "super user" actions within a specific environment.

  Once the permissions are gained, we can call an "package install" function. We can't call that function without the token type, so by being able to call it at all all we can assume we have permissions.

</details>
---
minutes: 5
---

# Debug

"Write to string" trait, for debug purposes.

Derivable: ✅ When to implement: Almost always

```rust
// pub trait Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
// }

#[derive(Debug)]
pub struct Date {
    day: u8,
    month: u8,
    year: i64,
}

#[derive(Debug)]
pub struct User {
    name: String,
    date_of_birth: Date,
}

pub struct PlainTextPassword {
    password: String,
    hint: String,
}

impl std::fmt::Debug for PlainTextPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlainTextPassword")
            .field("hint", &self.hint)
            .field("password", &"[omitted]")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        date_of_birth: Date { day: 31, month: 10, year: 2002 },
    };

    println!("{user:?}");
    println!(
        "{:?}",
        PlainTextPassword {
            password: "Password123".to_string(),
            hint: "Used it for years".to_string()
        }
    );
}
```

<details>
- Provides trivial "write to string" functionality.

- Formatting for _debug information_ for programmers during , not appearance or
  serialization.

- Allows for use of `{:?}` and `{#?}` interpolation in string formatting macros.

- When to not derive/implement: If a struct holds sensitive data, investigate if
  you should implement Debug for it.

  If Debug is needed, consider manually implementing Debug rather than deriving
  it. Omit the sensitive data from the implementation.

</details>

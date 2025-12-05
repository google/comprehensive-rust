---
minutes: 5
---

# TryFrom/TryInto

Fallible conversion from one type to another.

Derivable: ❌ When to implement: As-needed.

```rust
#[derive(Debug)]
pub struct InvalidNumber;

#[derive(Debug)]
pub struct DivisibleByTwo(usize);

impl TryFrom<usize> for DivisibleByTwo {
    type Error = InvalidNumber;
    fn try_from(value: usize) -> Result<Self, InvalidNumber> {
        if value.rem_euclid(2) == 0 {
            Ok(DivisibleByTwo(value))
        } else {
            Err(InvalidNumber)
        }
    }
}

fn main() {
    let success: Result<DivisibleByTwo, _> = 4.try_into();
    dbg!(success);
    let fail: Result<DivisibleByTwo, _> = 5.try_into();
    dbg!(fail);
}
```

<details>
- Provides conversion that can fail, returning a result type.

- Like `From`/`Into`, prefer implementing `TryFrom` for types rather than
  `TryInto`.

- Implementations can specify what the error type of the `Result`.

</details>

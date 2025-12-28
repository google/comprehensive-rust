---
minutes: 2
---

# `[method]_mut`: Mutable reference access

Suffix for access-style methods.

```rust,compile_fail
impl <T> Vec<T> {
    // Simplified
    fn get_mut(&mut self, usize) -> Option<&T>;
}

impl <T> [T] {
    // Simplified
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T>;
}

impl str {
    fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, Utf8Error>;
}
```

<details>
- Mut for Mutability

- Suffix that signifies the method gives access to a mutable reference.

  Requires mutable access to the value you're calling this method on.

</details>

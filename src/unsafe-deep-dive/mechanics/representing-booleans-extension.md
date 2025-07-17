# Extension

Create a similar data structure for Rust's [`char`] type. A `char` occupies 4
bytes, but not all 4 bytes sequences are valid as `char`.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html

Here is some starter code:

```rust
struct Char;

impl TryFrom<u32> for Char {
    type Error = u32;

    fn try_from(x: u32) -> std::result::Result<Self, <Self as TryFrom<u32>>::Error> {
        todo!() // Attempt conversion, returning Err(x) when invalid
    }
}

#[test]
fn repr_matches() {
    use std::alloc::Layout;

    assert_eq!(Layout::new::<char>(), Layout::new::<Char>());
}

#[test]
fn conversion() {
    for i in u32::MIN..=u32::MAX {
        let res = Char::try_from(i);

        match i {
            0..=0xD7FF | 0xE000..=0x10FFFF => assert!(res.is_ok()),
            _ => assert!(res.is_err()),
        };
    }
}
```

<details>

Representation:

From Rust's documentation:

> `char` is guaranteed to have the same size, alignment, and function call ABI
> as `u32` on all platforms.

</details>

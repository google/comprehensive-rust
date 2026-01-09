---
minutes: 5
---

# Encapsulated Unsafe Rust

```rust,editable
pub fn copy(dest: &mut [u8], source: &[u8]) {
    let len = dest.len().min(source.len());
    let mut i = 0;
    while i < len {
        // SAFETY: `i` must be in-bounds as it was produced by source.len()
        let new = unsafe { source.get_unchecked(i) };

        // SAFETY: `i` must be in-bounds as it was produced by dest.len()
        let old = unsafe { dest.get_unchecked_mut(i) };

        *old = *new;
        i += 1;
    }

    for (dest, src) in dest.iter_mut().zip(source) {
        *dest = *src;
    }
}

fn main() {
    let a = &[114, 117, 115, 116];
    let b = &mut [82, 85, 83, 84];

    println!("{}", String::from_utf8_lossy(b));
    copy(b, a);
    println!("{}", String::from_utf8_lossy(b));
}
```

<details>

“Here we have a safe function that encapsulates unsafe blocks that are used
internally.

“This implementation avoids iterators. Instead, the implementor is accessing
memory manually.”

“Is this correct?” “Are there any problems?”

“Who has responsibility for ensuring that correctness? The author of the
function.

“A Safe Rust function that contains unsafe blocks remains sound if it’s
impossible for an input to cause memory safety issues.

</details>

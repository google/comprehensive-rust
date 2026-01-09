---
minutes: 5
---

# Documented safety preconditions

```rust,editable
/// ...
///
/// # Safety
///
/// This function can easily trigger undefined behavior. Ensure that:
///
///  - `source` pointer is non-null and non-dangling
///  - `source` data ends with a null byte within its memory allocation
///  - `source` data is not freed (its lifetime invariants are preserved)
///  - `source` data contains fewer than `isize::MAX` bytes
pub unsafe fn copy(dest: &mut [u8], source: *const u8) {
    let source = {
        let mut len = 0;

        let mut end = source;
        // SAFETY: Caller has provided a non-null pointer
        while unsafe { *end != 0 } {
            len += 1;
            // SAFETY: Caller has provided a data with length < isize:MAX
            end = unsafe { end.add(1) };
        }

        // SAFETY: Caller maintains lifetime and aliasing requirements
        unsafe { std::slice::from_raw_parts(source, len + 1) }
    };

    for (dest, src) in dest.iter_mut().zip(source) {
        *dest = *src;
    }
}

fn main() {
    let a = [114, 117, 115, 116].as_ptr();
    let b = &mut [82, 85, 83, 84, 0];

    println!("{}", String::from_utf8_lossy(b));
    unsafe {
        copy(b, a);
    }
    println!("{}", String::from_utf8_lossy(b));
}
```

<details>

Changes to previous iterations:

- `copy` marked as unsafe
- Safety preconditions are documented
- inline safety comments

An unsafe function is sound when both its safety preconditions and its internal
unsafe blocks are documented.

Fixes needed in `main`.

- `a` does not satisfy one of the preconditions of `copy` (source` data ends
  with a null byte within its memory allocation)
- SAFETY comment needed

</details>

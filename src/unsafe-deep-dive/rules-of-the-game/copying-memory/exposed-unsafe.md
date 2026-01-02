---
minutes: 5
---

# Exposed Unsafe Rust

```rust,editable
pub fn copy(dest: &mut [u8], source: *const u8) {
    let source = {
        let mut len = 0;

        let mut end = source;
        while unsafe { *end != 0 } {
            len += 1;
            end = unsafe { end.add(1) };
        }

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
    copy(b, a);
    println!("{}", String::from_utf8_lossy(b));
}
```

<details>

onality of copying bytes from one place to the next remains the same.

“However, we need to manually create a slice. To do that, we first need to find
the end of the data.

“As we’re working with text, we’ll use the C convention of a null-terminated
string.

Compile the code. See that the output remains the same.

“An unsound function can still work correctly for some inputs. Just because your
tests pass, does not mean that you have a sound function.”

“Can anyone spot any issues?”

- Readability: difficult to quickly scan code
- `source` pointer might be null
- `source` pointer might be dangling, i.e. point to freed or uninitialized
  memory
- `source` might not be null-terminated

“Assume that we cannot change the function signature, what improvements could we
make to the code to address these issues?”

- Null pointer: Add null check with early return
  (`if source.is_null() { return; }`)
- Readability: Use a well-tested library rather than implementing “find first
  null byte” ourselves

“Some safety requirements are impossible to defensively check for, however,
i.e.:”

- dangling pointer
- no null termination byte

“How can we make this function sound?”

- Either
  - Change the type of the `source` input argument to something that has a known
    length, i.e. use a slice like the previous example.
- Or
  - Mark the function as unsafe
  - Document the safety preconditions

</details>

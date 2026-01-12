---
minutes: 8
---

# MaybeUninit and arrays

```rust
use std::mem::MaybeUninit;
use std::ptr;

fn main() {
    let input = b"RUST";

    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

    // Initialize elements by writing values to the memory
    for (i, input_byte) in input.iter().enumerate() {
        unsafe {
            let dst = buf.as_mut_ptr().add(i);
            ptr::write((*dst).as_mut_ptr(), *input_byte);
        }
    }

    // When a portion of an array is initialized, one can
    // use unsafe to isolate it
    let ptr_to_init_subslice = buf.as_ptr() as *const u8;
    let init =
        unsafe { std::slice::from_raw_parts(ptr_to_init_subslice, input.len()) };
    let text = std::str::from_utf8(init).unwrap();
    println!("{text}");

    // We must manually drop the initialized elements
    for element in &mut buf[0..input.len()] {
        unsafe {
            element.assume_init_drop();
        }
    }
}
```

<details>

To create an array of uninitialized memory, the `::uninit()` constructor can be
used within a `const` context.

Use `ptr::write` to initialize values as per normal.

`.assume_init()` does not work as easily for arrays. It requires every value to
be initialized, which may not occur when reusing a buffer. This example uses a
pointer to isolate the initialized bytes to create a string slice.

When creating a sub-slice of a partially-initialized array, be careful with
ownership and correctly implementing drop. Reminder: `MaybeUninit<T>` will not
call drop on its `T`.

`MaybeUninit<[u8;2048]>` is distinct from `[MaybeUninit::<u8>; 2048]`. This is
the difference between an array of uninitialized memory and an array that
contains uninitialized elements.

- `MaybeUninit<[u8;2048]>` is "all or nothing". You either fully initialize the
  whole array and then call `assume_init`, or you must keep it as
  `MaybeUninit<[u8; 2048]>` and avoid touching it as `[u8; 2048]`.
- `[MaybeUninit<u8>; 2048]` lets you initialize elements one at a time, then
  take a sub-slice of just the initialized prefix and treat it as `[u8]` via
  `std::slice::from_raw_parts`.
- `slice_assume_init_ref` is safe only when every element in the slice is
  initialized. For this example, we only pass `&buf[..input.len()]` after
  writing exactly those bytes.
- When `T` needs drop, you must manually call `assume_init_drop()` for the
  initialized elements. Skipping this leaks memory. However, calling it on an
  uninitialized element is undefined behavior.

</details>

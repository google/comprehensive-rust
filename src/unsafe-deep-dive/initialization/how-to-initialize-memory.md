---
minutes: 8
---

# How to Initialize Memory

Steps:

1. Create `MaybeUninit<T>`
2. Write a value to it
3. Notify Rust that the memory is initialized

```rust,editable
use std::mem::MaybeUninit;

fn main() {
    // Step 1: Create MaybeUninit
    let mut uninit = MaybeUninit::uninit();

    // Step 2: Write a valid value to the memory
    uninit.write(1);

    // Step 3: Inform the type system that the memory location is valid
    let init = unsafe { uninit.assume_init() };

    println!("{init}");
}
```

<details>

To work with uninitialized memory, follow this general workflow: create, write,
confirm.

1. Create `MaybeUninit<T>`. The `::uninit()` constructor is the most
   general-purpose one, but there are others which perform a write as well.

2. Write a value of T. Notice that this is available from safe Rust. Staying in
   safe Rust is useful because you must ensure that the value you write is
   valid.

3. Confirm to the type system that the memory is now initialized with the
   `.assume_init()` method.

</details>

---
minutes: 8
---

# Using an unsafe block

```rust,editable,ignore
fn main() {
    let numbers = vec![0, 1, 2, 3, 4];
    let i = numbers.len() / 2;

    let x = *numbers.get_unchecked(i);
    assert_eq!(i, x);
}
```

<details>

Walk through the code. Confirm that the audience is familiar with the
dereference operator.

Attempt to compile the code, trigger the compiler error.

Add the unsafe block:

```rust
# fn main() {
#     let numbers = vec![0, 1, 2, 3, 4];
#     let i = numbers.len() / 2;
# 
 let x = unsafe { *numbers.get_unchecked(i) };
#     assert_eq!(i, x);
# }
```

Prompt audience for a code review. Guide learners towards adding a safety
comment.

Add the safety comment:

```rust
// SAFETY: `i` must be within 0..numbers.len()
```

_Suggested Solution_

```rust
fn main() {
    let numbers = vec![0, 1, 2, 3, 4];
    let i = numbers.len() / 2;

    let x = unsafe { *numbers.get_unchecked(i) };
    assert_eq!(i, x);
}
```

</details>

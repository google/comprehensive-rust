---
minutes: 5
---

# Unsafe Functions

## Calling Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour:

```rust,editable
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("emoji: {}", emojis.get_unchecked(4..7));
        println!("emoji: {}", emojis.get_unchecked(7..11));
    }

    println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..7) }));

    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!("char count: {}", count_chars(unsafe {
    // emojis.get_unchecked(0..3) }));
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}
```

## Writing Unsafe Functions

You can mark your own functions as `unsafe` if they require particular
conditions to avoid undefined behaviour.

```rust,editable
/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 42;
    let mut b = 66;

    // Safe because ...
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}
```

<details>

## Calling Unsafe Functions

`get_unchecked`, like most `_unchecked` functions, is unsafe, because it can
create UB if the range is incorrect. `abs` is incorrect for a different reason:
it is an external function (FFI). Calling external functions is usually only a
problem when those functions do things with pointers which might violate Rust's
memory model, but in general any C function might have undefined behaviour under
any arbitrary circumstances.

The `"C"` in this example is the ABI;
[other ABIs are available too](https://doc.rust-lang.org/reference/items/external-blocks.html).

## Writing Unsafe Functions

We wouldn't actually use pointers for a `swap` function - it can be done safely
with references.

Note that unsafe code is allowed within an unsafe function without an `unsafe`
block. We can prohibit this with `#[deny(unsafe_op_in_unsafe_fn)]`. Try adding
it and see what happens. This will likely change in a future Rust edition.

</details>

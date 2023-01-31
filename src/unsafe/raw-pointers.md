# Dereferencing Raw Pointers

Creating pointers is safe, but dereferencing them requires `unsafe`:

```rust,editable
fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);  // Undefined behavior!
    }
}
```

<details>

It is good practice (and required by the Android Rust style guide) to write a comment for each
`unsafe` block explaining how the code inside it satisfies the safety requirements of the unsafe
operations it is doing.

In the case of pointer dereferences, this means that the pointers must be
[_valid_](https://doc.rust-lang.org/std/ptr/index.html#safety), i.e.:

 * The pointer must be non-null.
 * The pointer must be _dereferenceable_ (within the bounds of a single allocated object).
 * The object must not have been deallocated.
 * There must not be concurrent accesses to the same location.
 * If the pointer was obtained by casting a reference, the underlying object must be live and no
   reference may be used to access the memory.

In most cases the pointer must also be properly aligned.

Run the code above in [Miri] via the Playground to see it detect undefined
behavior. Miri is an interpreter for the mid-level intermediate representation
in the Rust compiler. It can detect some classes of undefined behavior.

[Miri]: https://github.com/rust-lang/miri

</details>

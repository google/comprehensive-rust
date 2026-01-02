# Determining Preconditions

Where do you find the safety preconditions?

```rust,editable
fn main() {
    let b: *mut i32 = std::ptr::null_mut();
    println!("{:?}", b.as_mut());
}
```

<details>

Attempt to compile the program to trigger the compiler error ("error\[E0133\]:
call to unsafe function ...").

Ask: “Where would you look if you wanted to know the preconditions for a
function? Here we need to understand when it's safe to convert from a null
pointer to a mutable reference.”

Locations to look:

- A function's API documentation, especially its safety section
- The source code and its internal safety comments
- Module documentation
- Rust Reference

Consult [the documentation] for the `as_mut` method.

Highlight Safety section.

> **Safety**
>
> When calling this method, you have to ensure that either the pointer is null
> or the pointer is convertible to a reference.

Click the "convertible to a reference" hyperlink to the "Pointer to reference
conversion"

Track down the rules for converting a pointer to a reference, aka is
"_deferencerable_".

Consider the implications of this excerpt (Rust 1.90.0) "You must enforce Rust’s
aliasing rules. The exact aliasing rules are not decided yet, ..."

</details>

[the documentation]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut

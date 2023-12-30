---
minutes: 10
---

# Dereferencing Raw Pointers

Creating pointers is safe, but dereferencing them requires `unsafe`:

```rust,editable
fn main() {
    let mut s = String::from("careful!");

    let r1 = &mut s as *mut String;
    let r2 = r1 as *const String;

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    // NOT SAFE. DO NOT DO THIS.
    /*
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3);
    */
}
```

<details>

It is good practice (and required by the Android Rust style guide) to write a
comment for each `unsafe` block explaining how the code inside it satisfies the
safety requirements of the unsafe operations it is doing.

In the case of pointer dereferences, this means that the pointers must be
[_valid_](https://doc.rust-lang.org/std/ptr/index.html#safety), i.e.:

- The pointer must be non-null.
- The pointer must be _dereferenceable_ (within the bounds of a single allocated
  object).
- The object must not have been deallocated.
- There must not be concurrent accesses to the same location.
- If the pointer was obtained by casting a reference, the underlying object must
  be live and no reference may be used to access the memory.

In most cases the pointer must also be properly aligned.

The "NOT SAFE" section gives an example of a common kind of UB bug: `*r1` has
the `'static` lifetime, so `r3` has type `&'static String`, and thus outlives
`s`. Creating a reference from a pointer requires _great care_.

</details>

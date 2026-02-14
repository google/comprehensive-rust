---
minutes: 10
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Dereferencing Raw Pointers

Creating pointers is safe, but dereferencing them requires `unsafe`:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut x = 10;

    let p1: *mut i32 = &raw mut x;
    let p2 = p1 as *const i32;

    // SAFETY: p1 and p2 were created by taking raw pointers to a local, so they
    // are guaranteed to be non-null, aligned, and point into a single (stack-)
    // allocated object.
    //
    // The object underlying the raw pointers lives for the entire function, so
    // it is not deallocated while the raw pointers still exist. It is not
    // accessed through references while the raw pointers exist, nor is it
    // accessed from other threads concurrently.
    unsafe {
        dbg!(*p1);
        *p1 = 6;
        // Mutation may soundly be observed through a raw pointer, like in C.
        dbg!(*p2);
    }

    // UNSOUND. DO NOT DO THIS.
    /*
    let r: &i32 = unsafe { &*p1 };
    dbg!(r);
    x = 50;
    dbg!(r); // Object underlying the reference has been mutated. This is UB.
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

The "UNSOUND" section gives an example of a common kind of UB bug: naïvely
taking a reference to the dereference of a raw pointer sidesteps the compiler's
knowledge of what object the reference is actually pointing to. As such, the
borrow checker does not freeze `x` and so we are able to modify it despite the
existence of a reference to it. Creating a reference from a pointer requires
_great care_.

</details>

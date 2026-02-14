---
minutes: 8
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Defining an unsafe function

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Convert a nullable pointer to a reference.
///
/// Returns `None` when `p` is null, otherwise wraps `val` in `Some`.
fn ptr_to_ref<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if ptr.is_null() {
        None
    } else {
        // SAFETY: `ptr` is non-null
        unsafe { Some(&mut *ptr) }
    }
}
```

<details>

“This looks as though it’s safe code, however it actually requires an unsafe
block.”

Highlight the dereference operation, i.e. `*p` within the unsafe block.

“Callers must ensure that the `ptr` is null, or that it may be converted to a
reference.

“It may be counter-intuitive, but many pointers cannot be converted to
references.

“Among other issues, a pointer could be created that points to some arbitrary
bits rather than a valid value. That’s not something that Rust allows and
something that this function needs to protect itself against.

“So we, as API designers, have two paths. We can either try to assume
responsibility for guarding against invalid inputs, or we can shift that
responsibility to the caller with the unsafe keyword.”

“The first path is a difficult one. We’re accepting a generic type T, which is
all possible types that implement Sized. That’s a lot of types!

“Therefore, the second path makes more sense.

_Extra content (time permitting)_

“By the way, if you’re interested in the details of pointers and what the rules
of converting them to references are, the standard library has a lot of useful
documentation. You should also look into the source code of many of the methods
on std::pointer.

“For example, the `ptr_to_ref` function on this slide actually exists in the
standard library as the `as_mut` method on pointers.”

Open the documentation for [std::pointer.as_mut] and highlight the Safety
section.

</details>

[std::pointer.as_mut]: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut

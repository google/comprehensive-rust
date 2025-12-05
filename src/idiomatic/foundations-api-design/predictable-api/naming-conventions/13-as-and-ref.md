---
minutes: 5
---

# As and Ref

As is a prefix for getting a reference to something internal. Ref is a suffix.

```rust,no_compile
Option::as_ref // &Option<T> -> Option<&T>
Option::as_slice // &Option<T> -> &[T] (0 or 1 elements)
OwnedFd::as_fd // &'a OwnedFd -> BorrowedFd<'a> (we'll see this later!)
Rc::as_ptr // &Rc<T> -> *const T
Vec::as_ptr // &Vec<T> -> *const T
```

<details>
- Method for getting a reference-style value from an owned or borrowed value.

- Often used for getting something internal to a type.

  Collection and smart pointer types often have an `as_ptr` method, giving
  access to the pointer of the value they contain.

- Does not consume the value! Main difference between this and `to` or `into`
  functions.

- Highlight: OwnedFd::as_fd duplicates a file descriptor while tying ownership
  of that descriptor to the original OwnedFd.

  This gets covered later! Don't worry about this for now.

- Ref is a common suffix with `as`-named functions.

  `as_ref` is often a transformation of a reference to a container type (such as
  option) to an owned container of reference types.

</details>

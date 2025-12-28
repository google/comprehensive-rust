---
minutes: 5
---

# `as_` and `_ref`: reference conversions

`as` is a prefix for methods that convert references. `ref` is a suffix (but
prefer `as`.)

`as` methods borrow out the primary piece of data contained in `&self`.

Most commonly return references, but can also return a custom borrowing type or
an unsafe pointer.

```rust,compile_fail
impl<T> Rc<T> {
    fn as_ptr(&self) -> *const T;

    // Very common on container types, see how it's also on Option.
    fn as_ref(&self) -> &T;
}

impl<T> Option<T> {
    fn as_ref(&self) -> Option<&T>;
    // Slices can be empty! So this is 0 or 1 elements.
    fn as_slice(&self) -> &[T];
}

impl OwnedFd {
    // Covered later.
    fn as_fd(&'a self) -> BorrowedFd<'a>;
}
```

<details>

- Method that returns a borrow of the primary piece of contained data.

- The borrowing relationship is most often straightforward: the return value is
  a reference that borrows `self`.

- Borrowing can also be subtle, and merely implied.

  - The returned value could be a custom borrowing type, fore example,
    `BorrowedFd` borrows `OwnedFd` through an explicit lifetime.

  - We cover custom borrowing types later in this deep dive,
    [PhantomData: OwnedFd & BorrowedFd](../../../leveraging-the-type-system/borrow-checker-invariants/phantomdata-04-borrowedfd.md).

  - The returned value could borrow `self` only logically, for example,
    `as_ptr()` methods return an unsafe pointer. The borrow checker does not
    track borrowing for pointers.

- The type implementing an "as" method should contain one primary piece of data
  that is being borrowed out.

  - The "as" naming convention does not work if the data type is an aggregate of
    many fields without an obvious primary one. Think about the call site:

  ```rust,compile_fail
  my_vec.as_ptr() // OK
  my_person.as_first_name() // does not read right, don't use "as_"
  my_person.first_name() // OK
  ```

  - If you want to have two getters that you need to distinguish, one that
    returns first name by value, and another one that returns it by reference,
    use `_ref` suffix:

  ```rust,compile_fail
  impl Person {
    fn first_name(&self) -> String
    fn first_name_ref() -> &str
    fn first_name_mut() -> &mut String
  }
  ```

</details>

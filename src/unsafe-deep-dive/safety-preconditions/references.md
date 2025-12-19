---
minutes: 10
---

# Example: references

```rust,editable
fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut i32;
    let b: *mut i32 = std::ptr::null_mut();

    println!("{:?}", *a);
    println!("{:?}", b.as_mut());
}
```

Confirm understanding of the syntax

`Box<i32>` type is a reference to an integer on the heap that is owned by the
box.

`*mut i32` type is a so-called raw pointer to an integer that the compiler does
not know the ownership of. Programmers need to ensure the rules are enforced
without assistance from the compiler.

a reference, i.e. `&mut i32`, means borrowed/

    - a raw pointer does not provide ownership info to Rust:
      - a pointer can be semantically owning the data, or semantically borrowing, 
        but that information only exists in the programmer's mind

- `&mut *boxed as *mut _` expression:
  - `*boxed` is ...
  - `&mut *boxed` is ...
  - finally, `as *mut i32` casts the reference to a pointer.

Confirm understanding of ownership

- Step through code
  - (Line 3) Creates raw pointer to the `123` by de-referencing the box,
    creating a new reference and casting the new reference as a pointer
  - (Line 4) Creates raw pointer with a NULL value
  - (Line 7) Converts the raw pointer to an Option with `.as_mut()`

- Highlight that pointers are nullable in Rust (unlike references).

- Compile to reveal the error messages

- Discuss
  - (Line 6) `println!("{:?}", *a);`
    - Prefix star dereferences a raw pointer
    - It is an explicit operation. Whereas regular references have implicit
      dereferencing most of the time thanks to the Deref trait. This is referred
      to as "auto-deref".
    - Dereferencing a raw pointer is an unsafe operation
    - Requires an unsafe block
  - (Line 7) `println!("{:?}", b.as_mut());`
    - `as_mut()` is an unsafe function.
    - Calling unsafe function requires an unsafe block

- Demonstrate: Fix the code (add unsafe blocks) and compile again to show the
  working program

- Demonstrate: Replace `as *mut i32` with `as *mut _`, show that it compiles.

  - We can partially omit the target type in the cast. The Rust compiler knows
    that the source of the cast is a `&mut i32`. This reference type can only be
    converted to one pointer type, `*mut i32`.

- Add safety comments
  - We said that the unsafe code marks the responsibility shift from the
    compiler to the programmer.
  - How do we convey that we thought about our unusual responsibilities while
    writing unsafe code? Safety comments.
  - Safety comments explain why unsafe code is correct.
  - Without a safety comment, unsafe code is not safe.

- Discuss: Whether to use one large unsafe block or two smaller ones
  - Possibility of using a single unsafe block rather than multiple
  - Using more allows safety comments as specific as possible

[ptr-as_mut]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_mut

_Suggested Solution_

```rust
fn main() {
    let mut boxed = Box::new(123);
    let a: *mut i32 = &mut *boxed as *mut i32;
    let b: *mut i32 = std::ptr::null_mut();

    // SAFETY: `a` is a non-null pointer to i32, it is initialized and still
    // allocated.
    println!("{:?}", unsafe { *a });

    // SAFETY: `b` is a null pointer, which `as_mut()` converts to `None`.
    println!("{:?}", unsafe { b.as_mut() });
}
```

</details>

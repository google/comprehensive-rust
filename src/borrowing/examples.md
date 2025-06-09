---
minutes: 3
---

# Borrow Errors

As a concrete example of how these borrowing rules prevent memory errors,
consider the case of modifying a collection while there are references to its
elements:

```rust,editable,compile_fail
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = &vec[2];
    vec.push(6);
    dbg!(elem);
}
```

We can also look at a case where these rules prevent incorrect optimizations:

```rust,editable,compile_fail
fn swap_and_increment(a: &mut i32, b: &mut i32) {
    *a = *a + 1;

    let tmp = *a;
    *a = *b;
    *b = tmp;

    *b = *a + 1;
}

fn main() {
    let mut x = 1;
    swap_and_increment(&mut x, &mut x);
}
```

<details>

- In the first case, modifying the collection by pushing new elements into it
  can potentially invalidate existing references to the collection's elements if
  the collection has to reallocate.

- In the second case, the aliasing rule prevents mis-compilation: In the C
  equivalent of this function, the program produces different results when
  compiled with optimzations enabled.

  - Show students [the C version of this on Godbolt][unoptimized]. By default,
    with no optimizations enabled it will print `x = 3`. But
    [enabling optimizations][optimized] causes it to instead print `x = 2`.

  - Note the use of the `restrict` keyword in the C example. This tells the
    compiler that `a` and `b` cannot alias, but nothing prevents you from
    violating that requirement. The compiler will silently produce the wrong
    behavior.

  - In the Rust version, using `&mut` automatically implies the same
    restriction, and the compiler statically prevents violations of this
    requirement.

  - Rust's aliasing rules provide strong guarantees about how references can
    alias, allowing the compiler to apply optimizations without breaking the
    semantics of your program.

</details>

[unoptimized]: https://godbolt.org/z/9EGh6eMxE
[optimized]: https://godbolt.org/z/Kxsf8sahT

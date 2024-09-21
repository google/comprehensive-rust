---
minutes: 5
---

# Generic Functions

Rust supports generics, which lets you abstract algorithms or data structures
(such as sorting or a binary tree) over the types used or stored.

```rust,editable
/// Pick `even` or `odd` depending on the value of `n`.
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a string: {:?}", pick(28, "dog", "cat"));
}
```

<details>

- Rust infers a type for T based on the types of the arguments and return value.

- In this example we only use the primitive types `i32` and `&str` for `T`, but
  we can use any type here, including user-defined types:

  ```rust,ignore
  struct Foo {
      val: u8,
  }

  pick(123, Foo { val: 7 }, Foo { val: 456 });
  ```

- This is similar to C++ templates, but Rust partially compiles the generic
  function immediately, so that function must be valid for all types matching
  the constraints. For example, try modifying `pick` to return `even + odd` if
  `n == 0`. Even if only the `pick` instantiation with integers is used, Rust
  still considers it invalid. C++ would let you do this.

- Generic code is turned into non-generic code based on the call sites. This is
  a zero-cost abstraction: you get exactly the same result as if you had
  hand-coded the data structures without the abstraction.

</details>

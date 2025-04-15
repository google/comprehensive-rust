---
minutes: 5
---

# Generic Functions

Rust supports generics, which lets you abstract algorithms or data structures
(such as sorting or a binary tree) over the types used or stored.

```rust,editable
fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}

fn main() {
    println!("picked a number: {:?}", pick(true, 222, 333));
    println!("picked a string: {:?}", pick(false, 'L', 'R'));
}
```

<details>

- It can be helpful to show the monomorphized versions of `pick`, either before
  talking about the generic `pick` in order to show how generics can reduce code
  duplication, or after talking about generics to show how monomorphization
  works.

  ```rust
  fn pick_i32(cond: bool, left: i32, right: i32) -> i32 {
      if cond { left } else { right }
  }

  fn pick_char(cond: bool, left: char, right: char) -> char {
      if cond { left } else { right }
  }
  ```

- Rust infers a type for T based on the types of the arguments and return value.

- In this example we only use the primitive types `i32` and `char` for `T`, but
  we can use any type here, including user-defined types:

  ```rust,ignore
  struct Foo {
      val: u8,
  }

  pick(false, Foo { val: 7 }, Foo { val: 99 });
  ```

- This is similar to C++ templates, but Rust partially compiles the generic
  function immediately, so that function must be valid for all types matching
  the constraints. For example, try modifying `pick` to return `left + right` if
  `cond` is false. Even if only the `pick` instantiation with integers is used,
  Rust still considers it invalid. C++ would let you do this.

- Generic code is turned into non-generic code based on the call sites. This is
  a zero-cost abstraction: you get exactly the same result as if you had
  hand-coded the data structures without the abstraction.

</details>

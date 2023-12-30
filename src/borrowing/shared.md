---
minutes: 10
---

# Borrowing a Value

As we saw before, instead of transferring ownership when calling a function, you
can let a function _borrow_ the value:

<!-- mdbook-xgettext: skip -->

```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}
```

- The `add` function _borrows_ two points and returns a new point.
- The caller retains ownership of the inputs.

<details>

This slide is a review of the material on references from day 1, expanding
slightly to include function arguments and return values.

# More to Explore

Notes on stack returns:

- Demonstrate that the return from `add` is cheap because the compiler can
  eliminate the copy operation. Change the above code to print stack addresses
  and run it on the [Playground] or look at the assembly in
  [Godbolt](https://rust.godbolt.org/). In the "DEBUG" optimization level, the
  addresses should change, while they stay the same when changing to the
  "RELEASE" setting:

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  #[derive(Debug)]
  struct Point(i32, i32);

  fn add(p1: &Point, p2: &Point) -> Point {
      let p = Point(p1.0 + p2.0, p1.1 + p2.1);
      println!("&p.0: {:p}", &p.0);
      p
  }

  pub fn main() {
      let p1 = Point(3, 4);
      let p2 = Point(10, 20);
      let p3 = add(&p1, &p2);
      println!("&p3.0: {:p}", &p3.0);
      println!("{p1:?} + {p2:?} = {p3:?}");
  }
  ```
- The Rust compiler can do return value optimization (RVO).
- In C++, copy elision has to be defined in the language specification because
  constructors can have side effects. In Rust, this is not an issue at all. If
  RVO did not happen, Rust will always perform a simple and efficient `memcpy`
  copy.

</details>

[Playground]: https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=0cb13be1c05d7e3446686ad9947c4671

# Lifetimes in Function Calls

In addition to borrowing its arguments, a function can return a borrowed value:

<!-- mdbook-xgettext: skip -->
```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = left_most(&p1, &p2);
    println!("p3: {p3:?}");
}
```

* `'a` is a generic parameter, it is inferred by the compiler.
* Lifetimes start with `'` and `'a` is a typical default name.
* Read `&'a Point` as "a borrowed `Point` which is valid for at least the
  lifetime `a`".
  * The _at least_ part is important when parameters are in different scopes.

<details>

In the above example, try the following:

* Move the declaration of `p2` and `p3` into a new scope (`{ ... }`), resulting in the following code:
  <!-- mdbook-xgettext: skip -->
  ```rust,ignore
  #[derive(Debug)]
  struct Point(i32, i32);

  fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
      if p1.0 < p2.0 { p1 } else { p2 }
  }

  fn main() {
      let p1: Point = Point(10, 10);
      let p3: &Point;
      {
          let p2: Point = Point(20, 20);
          p3 = left_most(&p1, &p2);
      }
      println!("p3: {p3:?}");
  }
  ```
  Note how this does not compile since `p3` outlives `p2`.

* Reset the workspace and change the function signature to `fn left_most<'a, 'b>(p1: &'a Point, p2: &'a Point) -> &'b Point`. This will not compile because the relationship between the lifetimes `'a` and `'b` is unclear.
* Another way to explain it:
  * Two references to two values are borrowed by a function and the function returns
    another reference.
  * It must have come from one of those two inputs (or from a global variable).
  * Which one is it? The compiler needs to know, so at the call site the returned reference is not used
    for longer than a variable from where the reference came from.

</details>

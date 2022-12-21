# Lifetimes in Function Calls

In addition to borrow its arguments, a function can return a borrowed value:

```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);  // Put into different scope
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}
```

* `'a` is a generic parameter, it is inferred by the compiler.
* Lifetimes start with `'` and `'a` is a typical default name.
* Read `&'a Point` as "a borrowed `Point` which is valid for at least the
  lifetime `a`".
  * The _at least_ part is important when parameters are in different scopes.

# Borrowing

Instead of transferring ownership when calling a function, you can let a
function _borrow_ the value:

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

* The `add` function _borrows_ two points and returns a new point.
* The caller retains ownership of the inputs.

---
minutes: 5
---

# Borrow One

In this example `find_nearest` takes in multiple borrows but returns only one of
them. The lifetime annotations explicitly tie the returned borrow to the
corresponding argument borrow.

```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

/// Searches `points` for the point closest to `query`.
/// Assumes there's at least one point in `points`.
fn find_nearest<'a>(points: &'a [Point], query: &Point) -> &'a Point {
    fn cab_distance(p1: &Point, p2: &Point) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    let mut nearest = None;
    for p in points {
        if let Some((_, nearest_dist)) = nearest {
            let dist = cab_distance(p, query);
            if dist < nearest_dist {
                nearest = Some((p, dist));
            }
        } else {
            nearest = Some((p, cab_distance(p, query)));
        };
    }

    nearest.map(|(p, _)| p).unwrap()
    // query // What happens if we do this instead?
}

fn main() {
    let points = &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1)];
    let query = Point(0, 2);
    let nearest = find_nearest(points, &query);

    // `query` isn't borrowed at this point.
    drop(query);

    dbg!(nearest);
}
```

<details>

- It may be helpful to collapse the definition of `find_nearest` to put more
  focus on the signature of the function. The actual logic in the function is
  somewhat complex and isn't important for the purpose of borrow analysis.

- When we call `find_nearest` the returned reference doesn't borrow `query`, and
  so we are free to drop it while `nearest` is still active.

- But what happens if we return the wrong borrow? Change the last line of
  `find_nearest` to return `query` instead. Show the compiler error to the
  students.

- The first thing we have to do is add a lifetime annotation to `query`. Show
  students that we can add a second lifetime `'b` to `find_nearest`.

- Show the new error to the students. The borrow checker verifies that the logic
  in the function body actually returns a reference with the correct lifetime,
  enforcing that the function adheres to the contract set by the function's
  signature.

# More to Explore

- The "help" message in the error notes that we can add a lifetime bound
  `'b:
  'a` to say that `'b` will live at least as long as `'a`, which would
  then allow us to return `query`. This is an example of lifetime "variance",
  which allows us to return a longer lifetime where a shorter one is expected.

- We can do something similar by returning a `'static` lifetime, e.g. a
  reference to a `static` variable. The `'static` lifetime is guaranteed to be
  longer than any other lifetime, so it's always safe to return in place of a
  shorter lifetime.

</details>

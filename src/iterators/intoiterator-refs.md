# `IntoIterator` On References

Implementing `IntoIterator` directly on a type means that creating the iterator
takes ownership of the object, so a common pattern is to implement it on a
reference to allow for the creation of borrowing iterators.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (&'a u32, &'a u32);
    type IntoIter = GridRefIter<'a>;
    fn into_iter(self) -> GridRefIter<'a> {
        GridRefIter { grid: self, i: 0, j: 0 }
    }
}

struct GridRefIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl<'a> Iterator for GridRefIter<'a> {
    type Item = (&'a u32, &'a u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((&self.grid.x_coords[self.i], &self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };

    // The loop can borrow the grid without taking ownership.
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }

    // The grid remains accessible after the loop.
    dbg!(&grid.x_coords);
}
```

<details>

- By implementing `IntoIterator` on `&Grid`, we can pass a reference into a
  `for` loop and get a borrowing iterator. This allows us to borrow the grid
  object in the loop without taking ownership of it.

- [This playground][1] shows that we can have both `GridIter` and `GridRefIter`
  at the same time.

- This pattern is used heavily in the standard library for its collection types,
  allowing for owning and borrowing iterators to be created implicitly based on
  whether you pass the object in by value, by reference, or by mutable
  reference.

- Pull up the [`IntoIterator`] docs and show the implementations for `[T; N]`,
  `&[T; N]`, and `&mut [T; N]`. Show that they produce different iterators with
  different item types:

  - The iterator for `[T; N]` produces `T` by value.
  - The iterator for `&[T; N]` produces `&T`.
  - The iterator for `&mut [T; N]` produces `&mut T`.

[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c3c29974bac8cc2065ca1b6608d5c8b9
[`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

</details>

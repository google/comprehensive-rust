---
minutes: 5
---

# `IntoIterator`

The `Iterator` trait tells you how to _iterate_ once you have created an
iterator. The related trait
[`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
defines how to create an iterator for a type. It is used automatically by the
`for` loop.

```rust,editable
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
```

<details>

- `IntoIterator` is the trait that makes for loops work. It is implemented by
  collection types such as `Vec<T>` and references to them such as `&Vec<T>` and
  `&[T]`. Ranges also implement it. This is why you can iterate over a vector
  with `for i in some_vec { .. }` but `some_vec.next()` doesn't exist.

Click through to the docs for `IntoIterator`. Every implementation of
`IntoIterator` must declare two types:

- `Item`: the type to iterate over, such as `i8`,
- `IntoIter`: the `Iterator` type returned by the `into_iter` method.

Note that `IntoIter` and `Item` are linked: the iterator must have the same
`Item` type, which means that it returns `Option<Item>`

The example iterates over all combinations of x and y coordinates.

Try iterating over the grid twice in `main`. Why does this fail? Note that
`IntoIterator::into_iter` takes ownership of `self`.

Fix this issue by implementing `IntoIterator` for `&Grid` and creating a
`GridRefIter` that iterates by reference. A version with both `GridIter` and
`GridRefIter` is available [in this playground][1].

The same problem can occur for standard library types: `for e in some_vector`
will take ownership of `some_vector` and iterate over owned elements from that
vector. Use `for e in &some_vector` instead, to iterate over references to
elements of `some_vector`.

</details>

[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=947e371c7295af758504f01f149023a1

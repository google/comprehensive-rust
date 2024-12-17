---
minutes: 5
---

# `Iterator` Trait

The [`Iterator`][1] trait defines how an object can be used to produce a
sequence of values. For example, if we wanted to create an iterator that can
produce the elements of a slice it might look something like this:

```rust,editable
struct SliceIter<'s> {
    slice: &'s [i32],
    i: usize,
}

impl<'s> Iterator for SliceIter<'s> {
    type Item = &'s i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.slice.len() {
            None
        } else {
            let next = &self.slice[self.i];
            self.i += 1;
            Some(next)
        }
    }
}

fn main() {
    let slice = [2, 4, 6, 8].as_slice();
    let iter = SliceIter { slice, i: 0 };
    for elem in iter {
        println!("elem: {elem}");
    }
}
```

<details>

- The `SliceIter` example implements the same logic as the C-style `for` loop
  demonstrated on the last slide.

- Point out to the students that iterators are lazy: Creating the iterator just
  initializes the struct but does not otherwise do any work. No work happens
  until the `next` method is called.

- Iterators don't need to be finite! It's entirely valid to have an iterator
  that will produce values forever. For example, a half open range like `0..`
  will keep going until integer overflow occurs.

## More to Explore

- The "real" version of `SliceIter` is the [`slice::Iter`][2] type in the
  standard library, however the real version uses pointers under the hood
  instead of an index in order to eliminate bounds checks.

- The `SliceIter` example is a good example of a struct that contains a
  reference and therefore uses lifetime annotations.

- You can also demonstrate adding a generic parameter to `SliceIter` to allow it
  to work with any kind of slice (not just `&[i32]`).

</details>

[1]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[2]: https://doc.rust-lang.org/stable/std/slice/struct.Iter.html

# `enumerate`

The [`enumerate`] helper gives us a way to get the index of each element for any
iterator.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let array = ['a', 'b', 'c', 'd', 'e'];

    // By default iterating over an array just gives its elements.
    for letter in array {
        println!("The element is {letter:?}, but what's its index?");
    }

    // `enumerate` gives both the element and its index.
    for (i, letter) in array.iter().enumerate() {
        println!("{letter:?} has index {i}");
    }
}
```

<details>

- A nuance of Rust's iterators and `for` loops is that an iterator by default
  does not give you the index of each element. This is fine when all you need
  are e.g. the values in an array, but it's common to also want the index of
  each element as you iterate.

- `enumerate` creates an iterator that produces a tuple where the first element
  is the index and the second element is the original iterator's item.

- Point out that we can use a pattern for the loop variable in a `for` loop.
  This allows us to destructure the tuple produced by `enumerate`.

- Note that we had to do `.iter()` first; `enumerate` is an iterator method, not
  a method on the array type itself.

</details>

[`enumerate`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.enumerate

# Day 1 Morning Exercises

## Arrays and `for` Loops

([back to exercise](for-loops.md))

```rust
{{#include for-loops.rs:solution}}
```
### Bonus question

It requires more advanced concepts. It might seem that we could use a slice-of-slices (`&[&[i32]]`) as the input type to transpose and thus make our function handle any size of matrix. However, this quickly breaks down: the return type cannot be `&[&[i32]]` since it needs to own the data you return.

You can attempt to use something like `Vec<Vec<i32>>`, but this doesn't work out-of-the-box either: it's hard to convert from `Vec<Vec<i32>>` to `&[&[i32]]` so now you cannot easily use `pretty_print` either.

Once we get to traits and generics, we'll be able to use the [`std::convert::AsRef`][1] trait to abstract over anything that can be referenced as a slice.

```rust
use std::convert::AsRef;
use std::fmt::Debug;

fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    // A line references a slice of items
    Line: AsRef<[T]>,
    // A matrix references a slice of lines
    Matrix: AsRef<[Line]>
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref());
    }
}

fn main() {
    // &[&[i32]]
    pretty_print(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    // [[&str; 2]; 2]
    pretty_print([["a", "b"], ["c", "d"]]);
    // Vec<Vec<i32>>
    pretty_print(vec![vec![1, 2], vec![3, 4]]);
}
```

In addition, the type itself would not enforce that the child slices are of the same length, so such variable could contain an invalid matrix.

[1]: https://doc.rust-lang.org/std/convert/trait.AsRef.html

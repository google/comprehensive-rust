# Day 1 Morning Exercises

## Arrays and `for` Loops

([back to exercise](for-loops.md))

```rust
{{#include for-loops.rs}}
```
### Bonus question

It honestly doesn't work so well. It might seem that we could use a slice-of-slices (`&[&[i32]]`) as the input type to transpose and thus make our function handle any size of matrix. However, this quickly breaks down: the return type cannot be `&[&[i32]]` since it needs to own the data you return.

You can attempt to use something like `Vec<Vec<i32>>`, but this doesn't work very well either: it's hard to convert from `Vec<Vec<i32>>` to `&[&[i32]]` so now you cannot easily use `pretty_print` either.

In addition, the type itself would not enforce that the child slices are of the same length, so such variable could contain an invalid matrix.

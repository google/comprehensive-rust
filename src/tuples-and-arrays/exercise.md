---
minutes: 30
---

# Exercise: Nested Arrays

Arrays can contain other arrays:

```rust
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
```

What is the type of this variable?

Use an array such as the above to write a function `transpose` which will
transpose a matrix (turn rows into columns):

<!-- mdbook-xgettext: skip -->

```bob
           ⎛⎡1 2 3⎤⎞      ⎡1 4 7⎤
"transpose"⎜⎢4 5 6⎥⎟  "=="⎢2 5 8⎥
           ⎝⎣7 8 9⎦⎠      ⎣3 6 9⎦
```

Hard-code both functions to operate on 3 × 3 matrices.

Copy the code below to <https://play.rust-lang.org/> and implement the
functions:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include exercise.rs:transpose}}
    unimplemented!()
}

{{#include exercise.rs:main}}
```

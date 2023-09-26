# Arrays and `for` Loops

We saw that an array can be declared like this:

```rust
let array = [10, 20, 30];
```

You can print such an array by asking for its debug representation with `{:?}`:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let array = [10, 20, 30];
    println!("array: {array:?}");
}
```

Rust lets you iterate over things like arrays and ranges using the `for`
keyword:

```rust,editable
fn main() {
    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in &array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}
```

Use the above to write a function `pretty_print` which pretty-print a matrix and
a function `transpose` which will transpose a matrix (turn rows into columns):

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

{{#include for-loops.rs:transpose}}
    unimplemented!()
}

{{#include for-loops.rs:pretty_print}}
    unimplemented!()
}

{{#include for-loops.rs:main}}
```

## Bonus Question

Could you use `&[i32]` slices instead of hard-coded 3 × 3 matrices for your
argument and return types? Something like `&[&[i32]]` for a two-dimensional
slice-of-slices. Why or why not?


See the [`ndarray` crate](https://docs.rs/ndarray/) for a production quality
implementation.

<details>

The solution and the answer to the bonus section are available in the 
[Solution](solutions-morning.md#arrays-and-for-loops) section.

The use of the reference `&array` within `for n in &array` is a subtle
preview of issues of ownership that will come later in the afternoon.

Without the `&`...
* The loop would have been one that consumes the array.  This is a
  change [introduced in the 2021
  Edition](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html).
* An implicit array copy would have occurred.  Since `i32` is a copy type, then
  `[i32; 3]` is also a copy type.

</details>

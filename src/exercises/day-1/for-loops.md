# Arrays and `for` Loops

We saw that an array can be declared like this:

```rust
let array = [10, 20, 30];
```

You can print such an array by asking for its debug representation with `{:?}`:

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
    for n in array {
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

```bob
           ⎛⎡1 2 3⎤⎞      ⎡1 4 7⎤
"transpose"⎜⎢4 5 6⎥⎟  "=="⎢2 5 8⎥
           ⎝⎣7 8 9⎦⎠      ⎣3 6 9⎦
```

Hard-code both functions to operate on 3 × 3 matrices.

Copy the code below to <https://play.rust-lang.org/> and implement the
functions:

```rust,should_panic
//fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//    unimplemented!()
//}

//fn pretty_print(matrix: &[[i32; 3]; 3]) {
//    unimplemented!()
//}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {matrix:?}" );
    //pretty_print(&matrix);

    //let transposed = transpose(matrix);
    //println!("transposed:");
    //pretty_print(&transposed);
}
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

</details>

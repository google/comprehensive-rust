# Arrays

```rust,editable
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}
```

<details>

- A value of the array type `[T; N]` holds `N` (a compile-time constant)
  elements of the same type `T`. Note that the length of the array is _part of
  its type_, which means that `[u8; 3]` and `[u8; 4]` are considered two
  different types. Slices, which have a size determined at runtime, are covered
  later.

- Try accessing an out-of-bounds array element. Array accesses are checked at
  runtime. Rust can usually optimize these checks away, and they can be avoided
  using unsafe Rust.

- We can use literals to assign values to arrays.

- The `println!` macro asks for the debug implementation with the `?` format
  parameter: `{}` gives the default output, `{:?}` gives the debug output. Types
  such as integers and strings implement the default output, but arrays only
  implement the debug output. This means that we must use debug output here.

- Adding `#`, eg `{a:#?}`, invokes a "pretty printing" format, which can be
  easier to read.

</details>

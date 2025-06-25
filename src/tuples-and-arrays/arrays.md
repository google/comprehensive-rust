---
minutes: 5
---

# Arrays

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}
```

<details>

- Arrays can also be initialized using the shorthand syntax, e.g. `[0; 1024]`.
  This can be useful when you want to initialize all elements to the same value,
  or if you have a large array that would be hard to initialize manually.

- A value of the array type `[T; N]` holds `N` (a compile-time constant)
  elements of the same type `T`. Note that the length of the array is _part of
  its type_, which means that `[u8; 3]` and `[u8; 4]` are considered two
  different types. Slices, which have a size determined at runtime, are covered
  later.

- Try accessing an out-of-bounds array element. The compiler is able to
  determine that the index is unsafe, and will not compile the code:

```rust,editable,compile_fail
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[6] = 0;
    println!("a: {a:?}");
}
```

- Array accesses are checked at runtime. Rust can usually optimize these checks
  away; meaning if the compiler can prove the access is safe, it removes the
  runtime check for better performance. They can be avoided using unsafe Rust.
  The optimization is so good that it's hard to give an example of runtime
  checks failing. The following code will compile but panic at runtime:

```rust,editable,should_panic
fn get_index() -> usize {
    6
}

fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[get_index()] = 0;
    println!("a: {a:?}");
}
```

- We can use literals to assign values to arrays.

- The `println!` macro asks for the debug implementation with the `?` format
  parameter: `{}` gives the default output, `{:?}` gives the debug output. Types
  such as integers and strings implement the default output, but arrays only
  implement the debug output. This means that we must use debug output here.

- Adding `#`, eg `{a:#?}`, invokes a "pretty printing" format, which can be
  easier to read.

</details>

# Shared and Unique Borrows

Rust puts constraints on the ways you can borrow values:

* You can have one or more `&T` values at any given time, _or_
* You can have exactly one `&mut T` value.

<!-- mdbook-xgettext: skip -->
```rust,editable,compile_fail
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}
```

<details>

* The above code does not compile because `a` is borrowed as mutable (through `c`) and as immutable (through `b`) at the same time.
* Move the `println!` statement for `b` before the scope that introduces `c` to make the code compile.
* After that change, the compiler realizes that `b` is only ever used before the new mutable borrow of `a` through `c`. This is a feature of the borrow checker called "non-lexical lifetimes".

</details>

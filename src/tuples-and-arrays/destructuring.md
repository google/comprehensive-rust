---
minutes: 5
---

# Patterns and Destructuring

When working with tuples and other structured values it's common to want to
extract the inner values into local variables. This can be done manually by
directly accessing the inner values:

```rust,editable
fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}
```

However, Rust also supports using pattern matching to destructure a larger value
into its constituent parts:

```rust,editable
fn print_tuple(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}
```

This works with any kind of structured value:

```rust,editable
struct Foo {
    a: i32,
    b: bool,
}

fn print_foo(foo: Foo) {
    let Foo { a, b } = foo;
    println!("a: {a}, b: {b}");
}
```

<details>

- The patterns used here are "irrefutable", meaning that the compiler can
  statically verify that the value on the right of `=` has the same structure as
  the pattern.
- A variable name is an irrefutable pattern that always matches any value, hence
  why we can also use `let` to declare a single variable.
- Rust also supports using patterns in conditionals, allowing for equality
  comparison and destructuring to happen at the same time. This form of pattern
  matching will be discussed in more detail later.
- Edit the examples above to show the compiler error when the pattern doesn't
  match the value being matched on.

</details>

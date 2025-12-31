---
minutes: 5
---

# Generic Function Parameters vs dyn Trait

We have two means of writing polymorphic functions, how do they compare?

```rust
fn print_display<T: std::fmt::Display>(t: &T) {
    println!("{}", t);
}

fn print_display_dyn(t: &dyn std::fmt::Display) {
    println!("{}", t);
}

fn main() {
    let int = 42i32;
    // Monomorphized to a unique function for i32 inputs.
    print_display(&int);
    // One per
    print_display_dyn(&int);
}
```

<details>

- We can write polymorphic functions over generics or over trait objects.

- When writing functions with generic parameters, for each unique type that
  substitutes a parameter a new version of that function is generated.

  We went over this in monomorphization: in exchange for binary size, we gain a
  greater capacity for optimization.

- When writing functions that take a trait object, only one version of that
  function will exist in the final binary (not counting inlining.)

- Generic parameters are zero-cost other than binary size. Types must be
  homogenous (all instances of T can only be the same type).

</details>

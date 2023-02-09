# Trait Bounds

When working with generics, you often want to require the types to implement
some trait, so that you can call this trait's methods.

You can do this with `T: Trait` or `impl Trait`:

```rust,editable
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// struct NotClonable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
}
```

<details>

Show a `where` clause, students will encounter it when reading code.
    
```rust,ignore
fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}
```

* It declutters the function signature if you have many parameters.
* It has additional features making it more powerful.
    * If someone asks, the extra feature is that the type on the left of ":" can be arbitrary, like `Option<T>`.
    
</details>

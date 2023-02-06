# Trait Bounds

When working with generics, you often want to limit the types. You can do this
with `T: Trait` or `impl Trait`:

```rust,editable
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// struct NotClonable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
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

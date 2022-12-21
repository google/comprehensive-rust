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
}
```

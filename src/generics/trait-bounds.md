# Trait Bounds

`T:` or `impl` 트레이트를 하용하여 제너릭을 사용할 때 타입을 제한할 수 있습니다:
> When working with generics, you often want to limit the types. You can do this
> with `T: Trait` or `impl Trait`:

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

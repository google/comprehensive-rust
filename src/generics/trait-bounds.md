---
minutes: 8
---

# Trait Bounds

When working with generics, you often want to require the types to implement
some trait, so that you can call this trait's methods.

You can do this with `T: Trait`:

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

- Try making a `NonClonable` and passing it to `duplicate`.

- When multiple traits are necessary, use `+` to join them.

- Show a `where` clause, students will encounter it when reading code.

  ```rust,ignore
  fn duplicate<T>(a: T) -> (T, T)
  where
      T: Clone,
  {
      (a.clone(), a.clone())
  }
  ```

  - It declutters the function signature if you have many parameters.
  - It has additional features making it more powerful.
    - If someone asks, the extra feature is that the type on the left of ":" can
      be arbitrary, like `Option<T>`.

- Note that Rust does not (yet) support specialization. For example, given the
  original `duplicate`, it is invalid to add a specialized `duplicate(a: u32)`.

</details>

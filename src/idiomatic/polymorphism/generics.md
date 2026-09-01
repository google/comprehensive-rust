# Generics

Generics are used when we want to abstract over types, but we expect the users
of our code to know the concrete types.

```rust
pub struct Vec<T> { ... }
```

```rust,editable
let ints: Vec<i32> = Vec::new();
vec.push(123);
vec.push(456);

let strings: Vec<&str> = Vec::new();
vec.push("hello");
vec.push("goodbye");
```

<details>

- Generics are our mechanism for **static polymorphism**, which is polymorphism
  where the types are fully known at compile time.

- One example of this is `Vec`, which is generic over the type of element it
  stores. `Vec` itself is polymorphic: It's written in such a way that it
  doesn't know what type of element will be stored in it. But in order to use a
  `Vec`, you must specify a concrete type to use for the element.

- Generics are a mechanism for **code reuse**: You have some common logic that
  is fundamentally the same regardless of what specific type it handles, and
  generics give you a way to abstract over those different types **without
  duplicating logic**.

- Note that there's no dynamism: The types must be fully known at compile time,
  there's no way to select a type for `Vec`'s element at runtime. This means
  that generics are not an option when we need **runtime polymorphism**. Later
  we will look at two mechanisms for doing dynamic polymorphism: Enums and
  `dyn`.

</details>

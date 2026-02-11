---
minutes: 1
---

# `new`: Constructor functions

Rust does not have a `new` keyword, instead `new` is a common prefix or whole
method name.

```rust,compile_fail
impl <T> Vec<T> {
    // Creates an empty vec.
    fn new() -> Vec<T>;
}

impl <T> Box<T> {
    fn new(T) -> Box<T>;
}
```

<details>

- There's no `new` keyword for Rust to initialize a new value, only functions
  you call or values you directly populate.

  `new` is conventional for the "default" constructor function for a type. It
  holds no special syntactic meaning.

  This is sometimes a prefix, it sometimes takes arguments.

</details>

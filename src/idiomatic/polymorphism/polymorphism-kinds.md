# Kinds of Polymorphism

In Rust we have 3 different mechanism for doing polymorphism:

- **Generics** - Static polymorphism where code abstracts over types but the
  types are fully known at compile time.
- **Enums** - Dynamic polymorphism where a fixed set of known types are selected
  between at runtime.
- **`dyn`** - Dynamic polymorphism where any type meeting a particular trait
  interface can be used.

<details>

- When discussing polymorphism in Rust, it's helpful to differentiate between
  **static** polymorphism and **dynamic** polymorphism.

  - **static polymorphism** is when we abstract over types, but the type
    information is fully known by the compiler. This allows us to reuse code in
    different type contexts without needing to add any runtime overhead, and we
    have access to the full set of features that traits expose. This is
    accomplished with **generics** in Rust.

  - **dynamic polymorphism** is when we need to select between different types
    at runtime, and we don't know at compile time which specific type will be
    used. When we have a fixed set of known types to choose from, we can use
    **enums** to track at runtime which one we have. When we don't know ahead of
    time which types may be used, e.g. if downstream users may introduce new
    types that we don't know about, then we use **`dyn`** to allow
    extensibility.

</details>

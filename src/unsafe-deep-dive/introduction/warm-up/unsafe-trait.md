---
minutes: 5
---

# Defining an unsafe trait

```rust,editable
/// Indicates that the type uses 32 bits of memory.
pub trait Size32 {}
```

<details>

“Now let’s define our own unsafe trait.”

Add the unsafe keyword and compile the code.

“If the requirements of the trait are semantic, then your trait may not need any
methods at all. The documentation is essential, however.”

“Traits without methods are called marker traits. When implementing them for
types, you are adding information to the type system. You have now given the
compiler the ability to talk about types that meet the requirements described in
the documentation.”

</details>

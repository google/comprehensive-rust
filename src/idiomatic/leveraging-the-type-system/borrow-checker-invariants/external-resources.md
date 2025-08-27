---
minutes: 0
---

# External Resources & Lifetime "Connections"

We can define types that capture lifetimes through parameters to prevent misuse.


```rust, editable
fn main() {



}
```

<details>

- BorrowedFd / OwnedFd uses this to model Unix-like resource handles (file descriptors), guaranteeing that while.

- These lifetimes are "held onto" with instances of `PhantomData`, a type that lets us have type parameters without needing "values" of that type.

- Lifetimes need to come from somewhere!

</details>
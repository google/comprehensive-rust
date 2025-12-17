# Unpin trait

- `Unpin` allows types within a `Pin` to move freely
- `!Unpin` types must never move

## PhantomPinned

```rust,ignore
pub struct PhantomPinned;

impl !Unpin for PhantomPinned {}
```

<details>

- Almost all types implement `Unpin`; compi automatically
- `Unpin` types can be moved even pinned
  - 'I promise I have no self-references, so moving me is always safe.'

Ask: What types might be `!Unpin`?

- Compiler-generated futures
- Types containing a `PhantomPinned` field
- Some types wrapping C++ objects

`!Unpin` types cannot be moved once pinned

</details>

[`Pantom`]: https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html

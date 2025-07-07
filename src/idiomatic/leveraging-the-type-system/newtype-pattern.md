---
minutes: 5
---

# Newtype Pattern

A _newtype_ is a wrapper around an existing type, often a primitive:

```rust
/// A unique user identifier, implemented as a newtype around `u64`.
pub struct UserId(u64);
```

Unlike type aliases, newtypes aren't interchangeable with the wrapped type:

```rust,compile_fail
# pub struct UserId(u64);
fn double(n: u64) -> u64 {
    n * 2
}

double(UserId(1)); // 🛠️❌
```

The Rust compiler won't implicitly convert to (or from) the underlying type.\
It won't let you use methods or operators defined on the underlying type either:

```rust,compile_fail
# pub struct UserId(u64);
assert_ne!(UserId(1), UserId(2)); // 🛠️❌
```

<details>

- Students should have encountered the newtype pattern in the "Fundamentals"
  course, when they learned about
  [tuple structs](../../user-defined-types/tuple-structs.md).

- Run the example to show students the error message from the compiler.

- Modify the example to use a typealias instead of a newtype, such as
  `type MessageId = u64`. The modified example should compile, thus highlighting
  the differences between the two approaches.

- Stress that newtypes, out of the box, have no behaviour attached to them. You
  need to be intentional about which methods and operators you are willing to
  forward from the underlying type. In our `UserId` example, it is reasonable to
  allow comparisons between `UserId`s, but it wouldn't make sense to allow
  arithmetic operations like addition or subtraction.

</details>

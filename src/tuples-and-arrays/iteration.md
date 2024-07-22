---
minutes: 3
---

# Array Iteration

The `for` statement supports iterating over arrays (but not tuples).

```rust,editable
fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
```

<details>

This functionality uses the `IntoIterator` trait, but we haven't covered that
yet.

The `assert_ne!` macro is new here. There are also `assert_eq!` and `assert!`
macros. These are always checked, while debug-only variants like `debug_assert!`
compile to nothing in release builds.

</details>

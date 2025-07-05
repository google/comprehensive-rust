---
minutes: 4
---

# Structs

Like tuples, Struct can also be destructured by matching:

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-structs.rs}}
```

<details>

- Change the literal values in `foo` to match with the other patterns.
- Add a new field to `Foo` and make changes to the pattern as needed.

## More to Explore

- Try `match &foo` and check the type of captures. The pattern syntax remains
  the same, but the captures become shared references. This is
  [match ergonomics](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html)
  and is often useful with `match self` when implementing methods on an enum.
  - The same effect occurs with `match &mut foo`: the captures become exclusive
    references.
- The distinction between a capture and a constant expression can be hard to
  spot. Try changing the `2` in the first arm to a variable, and see that it
  subtly doesn't work. Change it to a `const` and see it working again.

</details>

# Match Guards

When matching, you can add a _guard_ to a pattern. This is an arbitrary Boolean
expression which will be executed if the pattern matches:

```rust,editable
{{#include ../../third_party/rust-by-example/match-guards.rs}}
```

<details>

Match guards as a separate syntax feature are important and necessary. They are not
the same as separate `if` expression inside of the match arm.
  
An `if` expression inside of the branch block (after `=>`) happens after the match arm
is selected. Failing the `if` condition inside of that block won't result in other arms
of the original `match` expression being considered.
  
</details>

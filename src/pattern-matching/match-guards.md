# Match Guards

When matching, you can add a _guard_ to a pattern. This is an arbitrary Boolean
expression which will be executed if the pattern matches:

```rust,editable
{{#include ../../third_party/rust-by-example/match-guards.rs}}
```

<details>

Match guards as a separate syntax feature are important and necessary. They are not
the same as separate `if` statements inside of the match branch. 
  
`if` statement inside of the branch block (after `=>`) already happens after the match variant
is fully selected. Failing the `if` condition inside of that block won't be
able to "backtrack" and try to match other cases of the original `match` statement.
  
</details>

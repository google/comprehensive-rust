<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

- **Pattern Matching:** We use `match` to handle the different variants of the
  `Expression` enum. This ensures we cover all possible cases.
- **Destructuring:** The pattern `Expression::Op { op, left, right }`
  destructures the `Op` variant, giving us access to its inner fields.
- **Recursion:** Since `Expression` is a recursive data structure, `eval` is a
  recursive function. We call `eval(*left)` and `eval(*right)` to compute the
  values of the sub-expressions.
- **Smart Pointers (`Box`):** The `left` and `right` fields are of type
  `Box<Expression>`. `Box` puts the value on the heap. The `*` operator
  dereferences the box, moving the inner `Expression` out so it can be passed by
  value to the recursive `eval` calls.

<details>

- Mention that `Box` is necessary because `Expression` has infinite size
  otherwise (it contains itself). `Box` provides a fixed size (pointer size) for
  the recursive fields.
- Discuss integer division behavior (truncation) for `Operation::Div` since we
  are using integers.

</details>

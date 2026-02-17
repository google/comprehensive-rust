---
minutes: 15
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Nested Arrays

Arrays can contain other arrays:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
```

What is the type of this variable?

Use an array such as the above to write a function `transpose` that transposes a
matrix (turns rows into columns):

<!-- mdbook-xgettext: skip -->

```bob
           ⎛⎡1 2 3⎤⎞      ⎡1 4 7⎤
"transpose"⎜⎢4 5 6⎥⎟  "=="⎢2 5 8⎥
           ⎝⎣7 8 9⎦⎠      ⎣3 6 9⎦
```

Copy the code below to <https://play.rust-lang.org/> and implement the function.
This function only operates on 3×3 matrices.

```rust,should_panic,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:transpose}}
    todo!()
}

{{#include exercise.rs:main}}
```

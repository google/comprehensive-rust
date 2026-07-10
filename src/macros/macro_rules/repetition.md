---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Repetition

Declarative macros can match and expand lists of items of arbitrary length. This
is defined using the repetition syntax on both the left and right sides of the
match rules:

`$( ... ) SEPARATOR OPERATOR`

- **Separator:** An optional token separating each matched item (typically `,`
  or `;`).
- **Operators:**
  - `*`: Match zero or more repetitions.
  - `+`: Match one or more repetitions.
  - `?`: Match zero or one (optional).

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! print_all {
    // Match one or more expressions separated by commas:
    ( $( $val:expr ),+ ) => {
        // Expand the repetition:
        $(
            println!("Value: {}", $val);
        )*
    };
}

fn main() {
    print_all!(10, "hello", true, 3.14);
}
```

<details>

- Explain that the repetition block on the right-hand side `$( ... )*` must
  contain the exact meta-variables defined inside the left-hand side's
  repetition block.
- Note that the separator on the left-hand side (the comma `,` in
  `$( $val:expr ),+`), which is used to parse the input stream, may differ from
  that used on the right-hand side to join elements in the output stream. Here,
  the output separator is empty, resulting in sequential statements.
- You can use any punctuation token (except delimiters) as a separator.

</details>

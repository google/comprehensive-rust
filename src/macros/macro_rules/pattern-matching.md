---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Pattern Matching

You can match on exact, literal tokens inside macro patterns. This is extremely
useful for building custom DSLs or structured syntax matching:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! command {
    (start) => { println!("Starting..."); };
    (stop) => { println!("Stopping..."); };
    (restart) => {
        command!(stop);
        command!(start);
    };
}

fn main() {
    command!(restart);
}
```

In this example, the tokens `start`, `stop`, and `restart` are matched exactly
as literal identifiers. Literal identifiers and custom symbols (such as `=>`,
`,`, `:`, etc.) can be sequenced to design readable and mnemonic input syntax
for your macros.

Note, this macro invokes itself recursively in its transcriber body.

<details>

- Point out that because macros parse token streams, standard Rust rules (like
  variables needing to be declared or modules needing to exist) do not apply to
  the literal tokens being matched. They are treated purely as structural
  matches.
- Macros can invoke themselves and other macros, which can be useful for some
  more advanced macro design patterns which will be discussed later.

</details>

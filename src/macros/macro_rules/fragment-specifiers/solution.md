<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

Here is the implementation of the `pair!` macro using the `ty`, `pat`, and
`expr` fragment specifiers:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! pair {
    ($t:ty) => {
        ($t, $t)
    };
    ($p:pat) => {
        ($p, $p)
    };
    ($e:expr) => {
        ($e, $e)
    };
}

fn main() {
    let p: pair!(i32) = (10, 20);
    println!("Doubled type: {p:?}");

    let doubled = pair!(1 + 2);
    println!("Doubled expr: {doubled:?}");

    let val: pair!(i32) = pair!(7 as i32);

    match val {
        e @ pair!(7) => println!("Pattern position: matched {e:?}"),
        _ => println!("Did not match"),
    }
}
```

<details>

- **Context Flexibility:** Because declarative macros can expand in type,
  pattern, and expression positions, a single macro like `pair!` can be used in
  different locations in a codebase with the specific meaning varying based on
  its argument.
- There is something subtle happening here: the expansion depends on the input,
  not the context: `pair!(7)` always expands into a pattern, because `pat` is
  the first pattern that can match the token `7`. The expression invocation
  passes an `as` cast, which cannot appear in patterns.

</details>

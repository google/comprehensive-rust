---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# TT Bundling

When designing recursive or multi-layered declarative macros, passing around a
large number of parameters individually can make patterns messy and difficult to
maintain.

**TT Bundling** is a pattern where multiple arguments are grouped together
inside a single token group (e.g., `(param1: arg1, parmam2: arg2)`), allowing
them to be passed them through layers of recursion as a single unified `tt`
metavariable:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! check_for_error {
    // Base case 1: Unpack bundle to handle finding our needle (`error`) in the haystack
    ((present: $present:ident, absent: $absent:ident), error: $($tail:tt)*) => {
        $present(stringify!($($tail)*))
    };
    // Base case 2: Unpack bundle and handle end of input
    ((present: $present:ident, absent: $absent:ident), ) => {
        $absent()
    };
    // Recursive case: Match a non-`error` token and pass down the params bundle and rest of tokens to search
    ($bundle:tt, $_skip:tt $($tail:tt)*) => {
        check_for_error!($bundle, $($tail)*)
    };
}

fn fail(msg: &str) { eprintln!("FAIL: {msg}") }
fn pass() { eprintln!("PASS") }

fn main() {
    check_for_error!((present: fail, absent: pass), nothing is wrong here);
    check_for_error!((present: fail, absent: pass), but here is an error: uh-oh!);
}
```

The signature of the recursive arm does not change regardless of how complex the
bundled parameters may become.

<details>

- In the code example, `(present: fail, absent: pass)` represents the bundle,
  which is matched recursively as a single `$bundle:tt` and passed downstream.
- Macros with substantial state are most likely to benefit from this technique.

</details>

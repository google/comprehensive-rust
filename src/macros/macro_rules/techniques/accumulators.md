---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Push-Down Accumulators

Because all function-like macro calls must evaluate to valid Rust syntax (most
frequently, expressions), we cannot directly generate and return loose token
lists like `a, b, c` from helper macros to be embedded inside templates.

A **Push-Down Accumulator** is a pattern designed to bypass this expression
constraint by recursively accumulating tokens inside a dedicated buffer
structure (such as a brackted list) and feeding that buffer into subsequent
macro calls:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! init_array {
    [$e:expr; $n:tt] => {
        accum!([$n, $e] -> [])
    };
}

macro_rules! accum {
    ([3, $e:expr] -> [$($body:tt)*]) => { accum!([2, $e] -> [$($body)* $e,]) };
    ([2, $e:expr] -> [$($body:tt)*]) => { accum!([1, $e] -> [$($body)* $e,]) };
    ([1, $e:expr] -> [$($body:tt)*]) => { accum!([0, $e] -> [$($body)* $e,]) };
    ([0, $_:expr] -> [$($body:tt)*]) => { [$($body)*] };
}

fn main() {
    let arr = init_array![String::from("hi!"); 3];
    println!("Array: {:?}", arr);
}
```

- **Token Accumulation:** The right-hand side `[]` buffer accumulates expression
  clones sequentially.
- **AST Soundness:** Every intermediate macro recursion is structured as a valid
  macro call expression, which keeps the AST parser happy.

<details>

- Explain why helper macros like `params!() => { "foo", "bar" }` fail: the
  compiler expects the macro to expand into a single valid expression, but a
  list of comma-separated expressions is not a single expression.
- Show how the push-down accumulator avoids this: `init_array!` delegates to
  `accum!`, which keeps passing a structured macro call containing a growing
  list inside `[]` until it reaches the base case `[0, ...]`, which finally
  returns the completed array `[...]`.
- Mention that this pattern is exceptionally powerful for complex template
  formatting, but has a high cognitive overhead to write and maintain.

</details>

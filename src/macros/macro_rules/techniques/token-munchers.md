---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Token Munchers

An **Incremental TT Muncher** is a recursive macro pattern that processes input
token streams by "eating" (matching) one or more tokens off the front of the
stream, and recursively passing the remainder back to itself.

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! chain {
    // Base case: Only one method call remains
    ($self:expr, $fn_name:ident $(,)?) => {
        $self.$fn_name()
    };
    // Recursive case: Munch a method call off the front and pass the rest
    ($self:expr, $fn_name:ident, $($tail:tt)*) => {
        chain!($self.$fn_name(), $($tail)*)
    };
}

fn main() {
    let s = String::from("Hello, World");
    // Expands recursively to: s.len().trailing_zeros()
    let zeros = chain!(s, len, trailing_zeros);
    println!("Trailing zeros in length: {}", zeros);
}
```

TT munchers can parse complex custom grammars, but on the other hand, they can
be difficult to read and debug. For small embedded DSLs, they are probably the
right choice, but for more complex parsers other techniques may be more
appropriate.

## Limitations

Because the recursive call passes down the entire rest of the input, while each
step only removes a fixed-size portion of it, TT munchers are quadratic in the
size of their input. For large inputs, compile times can bloat, and you might
exceed the compiler's default macro recursion limit.

There are also some limits around the compiler's built-in follow-set
restrictions (on which fragment specifiers are allowed immediately preceding a
trailing `tt`), needed to prevent parser ambiguity. If we follow the pattern of
sticking to `tt` matchers on the left, we mostly avoid this limitation.

<details>

- Explain that the `tt` specifier is key here because it matches any single
  token tree (including identifiers, literals, or bracketed groups), allowing
  the recursive call to accept "anything else" in `$tail`.
- Note how `chain!(s, len, trailing_zeros)` matches the recursive case,
  expanding to `chain!(s.len(), trailing_zeros)`, which then matches the base
  case, expanding to `s.len().trailing_zeros()`.
- Warn about rule ordering: a repeated `tt` matcher will also match a single
  `tt`, so rules expecting the latter should come first.
- Discuss tradeoffs: if a macro is complex enough to require a TT muncher, there
  is a chance it may be better expressed as a procedural macro.

</details>

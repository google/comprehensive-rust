---
minutes: 20
---

# Exercise: Rewriting with Result

The following implements a very simple parser for an expression language.
However, it handles errors by panicking. Rewrite it to instead use idiomatic
error handling and propagate errors to a return from `main`. Feel free to use
`thiserror` and `anyhow`.

HINT: start by fixing error handling in the `parse` function. Once that is
working correctly, update `Tokenizer` to implement
`Iterator<Item=Result<Token, TokenizerError>>` and handle that in the parser.

```rust,editable
{{#include exercise.rs:types}}

{{#include exercise.rs:panics}}
```

---
minutes: 30
---

# Exercise: ROT13

In this example, you will implement the classic
["ROT13" cipher](https://en.wikipedia.org/wiki/ROT13). Copy this code to the
[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d3ad23a195b8c8f56a007cc6b1ab8eb8)
, and implement the missing bits. Only rotate ASCII alphabetic
characters, to ensure the result is still valid UTF-8.

```rust,editable
{{#include exercise.rs:head }}

// Implement the `Read` trait for `RotDecoder`.

{{#include exercise.rs:tests }}
```

What happens if you chain two `RotDecoder` instances together, each rotating by
13 characters?

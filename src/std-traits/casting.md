---
minutes: 5
---

# Casting

Rust has no _implicit_ type conversions, but does support explicit casts with
`as`. These generally follow C semantics where those are defined.

```rust,editable
fn main() {
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}
```

The results of `as` are _always_ defined in Rust and consistent across
platforms. This might not match your intuition for changing sign or casting to a
smaller type -- check the docs, and comment for clarity.

Casting with `as` is a relatively sharp tool that is easy to use incorrectly,
and can be a source of subtle bugs as future maintenance work changes the types
that are used or the ranges of values in types. Casts are best used only when
the intent is to indicate unconditional truncation (e.g. selecting the bottom 32
bits of a `u64` with `as u32`, regardless of what was in the high bits).

For infallible casts (e.g. `u32` to `u64`), prefer using `From` or `Into` over
`as` to confirm that the cast is in fact infallible. For fallible casts,
`TryFrom` and `TryInto` are available when you want to handle casts that fit
differently from those that don't.

<details>

Consider taking a break after this slide.

`as` is similar to a C++ static cast. Use of `as` in cases where data might be
lost is generally discouraged, or at least deserves an explanatory comment.

This is common in casting integers to `usize` for use as an index.

</details>

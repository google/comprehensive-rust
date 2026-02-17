<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Unpin trait

- `Unpin` type allows types to move freely, even when they're wrapped by a `Pin`
- Most types implement `Unpin`, because it is an "`auto trait`"
- `auto trait` behavior can be changed:
  - `!Unpin` types must never move
  - Types containing a `PhantomPinned` field do not implement `Unpin` by default

<details>

Explain that when a trait implements `Unpin`, the pinning behavior of `Pin<Ptr>`
does not get invoked. The value is free to move.

Explain that almost all types implement `Unpin`; automatically implemented by
the compiler.

Types implementing `Unpin` are saying: 'I promise I have no self-references, so
moving me is always safe.'

Ask: What types might be `!Unpin`?

- Compiler-generated futures
- Types containing a `PhantomPinned` field
- Some types wrapping C++ objects

`!Unpin` types cannot be moved once pinned

</details>

[`PhantomPinned`]: https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html

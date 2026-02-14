---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Implementing Branded Types (Branding 3/4)

Constructing branded types is different to how we construct non-branded types.

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
# use std::marker::PhantomData;
#
# #[derive(Default)]
# struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);
struct ProvenIndex<'id>(usize, InvariantLifetime<'id>);

struct Bytes<'id>(Vec<u8>, InvariantLifetime<'id>);

impl<'id> Bytes<'id> {
    fn new<T>(
        // The data we want to modify in this context.
        bytes: Vec<u8>,
        // The function that uniquely brands the lifetime of a `Bytes`
        f: impl for<'a> FnOnce(Bytes<'a>) -> T,
    ) -> T {
        f(Bytes(bytes, InvariantLifetime::default()),)
    }

    fn get_index(&self, ix: usize) -> Option<ProvenIndex<'id>> {
        if ix < self.0.len() { Some(ProvenIndex(ix, InvariantLifetime::default())) }
        else { None }
    }
    
    fn get_proven(&self, ix: &ProvenIndex<'id>) -> u8 {
        debug_assert!(ix.0 < self.0.len());
        unsafe { *self.0.get_unchecked(ix.0) } 
    }
}
```

<details>

- Motivation: We want to have "proven indexes" for a type, and we don't want
  those indexes to be usable by different variables of the same type. We also
  don't want those indexes to escape a scope.

  Our Branded Type will be `Bytes`: a byte array.

  Our Branded Token will be `ProvenIndex`: an index known to be in range.

- There are several notable parts to this implementation:
  - `new` does not return a `Bytes`, instead asking for "starting data" and a
    use-once Closure that is passed a `Bytes` when it is called.
  - That `new` function has a `for<'a>` on its trait bound.
  - We have both a getter for an index and a getter for a values with a proven
    index.

- Ask: Why does `new` not return a `Bytes`?

  Answer: Because we need `Bytes` to have a unique lifetime controlled by the
  API.

- Ask: So what if `new()` returned `Bytes`, what is the specific harm that it
  would cause?

  Answer: Think about the signature of that hypothetical `new()` method:

  `fn new<'a>() -> Bytes<'a> { ... }`

  This would allow the API user to choose what the lifetime `'a` is, removing
  our ability to guarantee that the lifetimes between different instances of
  `Bytes` are unique and unable to be subtyped to one another.

- Ask: Why do we need both a `get_index` and a `get_proven`?

  Expect "Because we can't know if an index is occupied at compile time"

  Ask: Then what's the point of the proven indexes?

  Answer: Avoiding bounds checking while keeping knowledge of what indexes are
  occupied specific to individual variables, unable to erroneously be used on
  the wrong one.

  Note: The focus is not on only on avoiding overuse of bounds checks, but also
  on preventing that "cross over" of indexes.

</details>

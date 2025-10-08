---
minutes: 10
---

# Implementing Branded Types (Branding 3/4)

Constructing branded types is different to how we construct non-branded types.

```rust
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
    
    fn get_proven(&self, ix: &ProvenIndex<'id>) -> u8 { self.0[ix.0] }
}
```

<details>

- Motivation: We want to have "proven indexes" for a type, and we don't want
  those indexes to be usable by different variables of the same type. We also
  don't want those indexes to escape a scope.

  Our Branded Type will be `Bytes`: a byte array.

  Our Branded Token will be `ProvenIndex`: an index known to be occupied.

- There are several notable parts to this implementation:
  - `new` does not return a `Bytes`, instead asking for "starting data" and a
    use-once Closure that is passed a `Bytes` when it is called.
  - That `new` function has a `for<'a>` on its trait bound.
  - We have both a getter for an index and a getter for a values with a proven
    index.

- Ask: Why does `new` not return a `Bytes`?

  Answer: At least partially because we need to introduce a lifetime to `Bytes`.

- Ask: Why do we need both a `get_index` and a `get_proven`?

  Expect "Because we can't know if an index is occupied at compile time"

  Ask: Then what's the point of the proven indexes?

  Answer: The throughline of preventing proven indexes "crossing over" to arrays
  of the same type, causing panics.

  Note: The focus is not on avoiding overuse of bounds checks, but instead on
  preventing that "cross over" of indexes.

</details>

---
minutes: 10
---

# Comparisons

These traits support comparisons between values. All traits can be derived for
types containing fields that implement these traits.

## `PartialEq` and `Eq`

`PartialEq` is a partial equivalence relation, with required method `eq` and
provided method `ne`. The `==` and `!=` operators will call these methods.

```rust,editable
struct Key {
    id: u32,
    metadata: Option<String>,
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

`Eq` is a full equivalence relation (reflexive, symmetric, and transitive) and
implies `PartialEq`. Functions that require full equivalence will use `Eq` as a
trait bound.

## `PartialOrd` and `Ord`

`PartialOrd` defines a partial ordering, with a `partial_cmp` method. It is used
to implement the `<`, `<=`, `>=`, and `>` operators.

```rust,editable
use std::cmp::Ordering;
#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}
impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}
```

`Ord` is a total ordering, with `cmp` returning `Ordering`.

<details>

`PartialEq` can be implemented between different types, but `Eq` cannot, because
it is reflexive:

```rust,editable
struct Key {
    id: u32,
    metadata: Option<String>,
}
impl PartialEq<u32> for Key {
    fn eq(&self, other: &u32) -> bool {
        self.id == *other
    }
}
```

In practice, it's common to derive these traits, but uncommon to implement them.

</details>

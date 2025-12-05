---
minutes: 10
---

# Why and What, not How and Where

Avoid documenting irrelevant details that may frequently change.

```rust,no_compile
/// Sorts a slice. Implemented using recursive quicksort.
fn sort_quickly<T: Ord>(to_sort: &mut [T]) { /* ... */
}

/// Calls an org-internal service using reqwest.
fn ask_service(url: &str) -> String { /* ... */
}
```

<details>
- Motivation: Using doc comments to explain how a function does something internally means if that internal implementation detail changes, the doc comment needs to change as well.

Internal information is likely irrelevant to a user. Imagine explaining in a doc
comment for a function that you're using for loops to solve a problem, what is
the point of this information?

- It could be that the implementation is necessary to explain, but this is
  likely due to whatever effects or invariants the user of that API needs to be
  aware of instead.

  Focus on those effects and invariants instead of instead of the implementation
  details themselves.

  Reiterate: Implementation details can and will change, so do not explain these
  details.

  TODO: Real-life example of something appropriate to a large system.

- Don't talk about where something is used for the sake of it.

  This is another instance where this information can become stale quickly.

- Prefer instead to focus on what the function does (though again, not how it is
  implemented) for a user trying to reach this practical information as quickly
  as possible.

</details>

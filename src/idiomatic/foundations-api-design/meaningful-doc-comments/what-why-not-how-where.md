---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Why and What, not How and Where

Avoid documenting irrelevant details that may frequently change.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Sorts a slice. Implemented using recursive quicksort.

fn sort_quickly<T: Ord>(to_sort: &mut [T]) { /* ... */
}

// bad
/// Saves a `User` record to the Postgres database.  
///  
/// This function opens a new connection and begins a transaction. It checks  
/// if a user with the given ID exists with a `SELECT` query. If a user is  
/// not found, performs an `INSERT`.  
///  
/// # Errors  
///  
/// Returns an error if any database operation fails.  
pub fn save_user(user: &User) -> Result<(), db::Error> {
    // ...
}

// good
/// Atomically saves a user record.  
///  
/// # Errors  
///  
/// Returns a `db::Error::DuplicateUsername` error if the user (keyed by  
/// `user.username` field) already exists.  
pub fn save_user(user: &User) -> Result<(), db::Error> {
    // ...
}
```

<details>

- Motivation: Users want to know the contract of the API (what is guaranteed
  about this function), rather than implementation details.

- Motivation: Doc comments that explain implementation details become outdated
  faster than comments that explain the contract.

  Internal information is likely irrelevant to a user. Imagine explaining in a
  doc comment for a function that you're using for loops to solve a problem,
  what is the point of this information?

- Consider the `sort_quickly` function above. Its documentation calls out that
  it uses quicksort, but is this necessary?

  It could be that another sorting function is used in the future, if that were
  the case then this comment would need to be updated too. This is a point of
  failure in documentation.

- It could be that the implementation is necessary to explain, but this is
  likely due to whatever effects or invariants the user of that API needs to be
  aware of instead.

  Focus on those effects and invariants instead of instead of the implementation
  details themselves.

  Reiterate: Implementation details can and will change, so do not explain these
  details.

- Don't talk about where something is used for the sake of it.

  This is another instance where this information can become stale quickly.

- Focus on what the function does (not how it is implemented) for a user trying
  to reach this practical information as quickly as possible.

</details>

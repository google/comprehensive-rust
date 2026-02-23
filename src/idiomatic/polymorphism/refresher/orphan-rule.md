---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Orphan Rule

What prevents users from writing arbitrary trait implementations for any type?

```rust,editable,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// Crate `postgresql-bindings`

pub struct PostgresqlConn(/* details */);

// Crate `database-traits`, depends on `postgresql-bindings`

pub trait DbConnection {
    /* methods */
}

impl DbConnection for PostgresqlConn {} // ✅, `DbConnection` is local.

// Crate `mycoolnewdb` depends on `database-traits`

pub struct MyCoolNewDbConn(/* details */);

impl DbConnection for MyCoolNewDbConn {} // ✅, `MyCoolNewDbConn` is local.

// Neither `PostgresqlConn` or `DbConnection` are local to `mycoolnewdb`.
// This would lead to two implementations of `DbConnection` for PostgresqlConn!
impl DbConnection for PostgresqlConn {} // ❌🔨
```

<details>

- Rust traits should never be able to be implemented twice in its ecosystem. Two
  implementations of the same trait for the same type is a conflict with no
  solution.

- We can prevent this within a crate by detecting if there are multiple
  definitions and disallowing it, but what about between crates in the entire
  Rust ecosystem?

- Types are either _local_ to a crate, they are defined there, or they're not.

  In the example's "crates", `PostgresqlConn` is local to `postgresql-bindings`,
  `MyCoolNewDbConn` is local to `mycoolnewdb`.

- Traits are also either _local_ to a crate, they are defined there, or they're
  not.

  Again in the example, the `DbConnection` trait is local to `database-traits`.

- If something is local, you can write trait implementations for it.

  If the trait is local, you can write implementations of that trait for any
  type.

  If the type is local, you can write any trait implementations for that type.

- Outside of these boundaries, trait implementations cannot be written.

  This keeps implementations "coherent": Only one implementation of a trait for
  a type can exist across crates.

ref:

- https://doc.rust-lang.org/stable/reference/items/implementations.html#r-items.impl.trait.orphan-rule

</details>

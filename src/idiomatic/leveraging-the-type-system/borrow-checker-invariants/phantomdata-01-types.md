---
minutes: 5
---

# PhantomData 1/4: De-duplicating Same Data & Semantics

The newtype pattern can sometimes come up against the DRY principle, how do we
solve this?

<!-- dprint-ignore-start -->
```rust,editable,compile_fail
pub struct UserId(u64);
impl ChatUser for UserId { /* ... */ }

pub struct PatronId(u64);
impl ChatUser for PatronId { /* ... */ }

pub struct ModeratorId(u64);
impl ChatUser for ModeratorId { /* ... */ }
impl ChatModerator for ModeratorId { /* ... */ }

pub struct AdminId(u64);
impl ChatUser for AdminId { /* ... */ }
impl ChatModerator for AdminId { /* ... */ }
impl ChatAdmin for AdminId { /* ... */ }

// And so on ...
fn main() {}
```
<!-- dprint-ignore-end -->

<details>

- Problem: We want to use the newtype pattern to differentiate permissions, but
  we're having to implement the same traits over and over again for the same
  data.

- Ask: Assume the details of each implementation here are the same between
  types, what are ways we can avoid repeating ourselves?

  Expect:
  - Make this an enum, not distinct data types.
  - Bundle the user ID with permission tokens like
    `struct Admin(u64, UserPermission, ModeratorPermission, AdminPermission);`
  - Adding a type parameter which encodes permissions.
  - Mentioning `PhantomData` ahead of schedule (it's in the title).

</details>

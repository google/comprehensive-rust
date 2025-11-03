---
minutes: 5
---

# PhantomData 1/2: Type-level tagging

<!-- dprint-ignore-start -->
```rust,editable
// use std::marker::PhantomData;

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

</details>

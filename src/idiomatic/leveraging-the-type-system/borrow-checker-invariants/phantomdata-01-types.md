---
minutes: 15
---

# PhantomData 1/2: Type-level tagging

<!-- dprint-ignore-start -->
```rust,editable,compile_fail
// use std::marker::PhantomData;

pub trait ChatUser {}
pub trait ChatModerator {}
pub trait ChatAdmin {}

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
  we're duplicating identical implementations for identical data.

- Motivation: We want to be able to "tag" structures with different type
  parameters as a way to tell them apart or pass on lifetime information to
  them.

  See: [Typestate Generics](../typestate-pattern/typestate-generics.md) for
  instances of telling apart different data relevant to stages of an algorithm
  with type parameter differences.

  In practice, these "tags" tend to be zero-sized types. What they mean will
  depend on the shape and context of the API they're a part of.

- Demonstrate: Change the implementation to the following:

  <!-- dprint-ignore-start -->
  ```rust
  pub struct ChatId<T> { id: u64, tag: T }

  pub struct UserTag;
  pub struct PatronTag;
  pub struct ModeratorTag;
  pub struct AdminTag;

  impl ChatId<UserTag> { /* ... */ }
  impl ChatId<PatronTag> { /* ... */ }
  impl ChatId<ModeratorTag> { /* ... */ }
  impl ChatId<AdminTag> { /* ... */ }
  ```
  <!-- dprint-ignore-end -->

  Ask: What issues does having it be an actual instance of that type pose?

  Answer: If it's not a zero-sized type (like `()` or `struct MyTag;`), then
  we're allocating more memory than we need to when all we care for is type
  information that is only relevant at compile-time.

  This also makes initializing the data a pain for users and the maintainers of
  the library alike, as users need to manually create a value and pass that to
  whatever constructors are exposed.

- Demonstrate: in `main`, show how users of this API need to pass values of the
  "tag" types. Construct values with the `Tag` types as above, then try to
  construct values such as `ChatId<String>` or
  `ChatId<(UserTag, PatronTag, ModeratorTag, AdminTag, Vec<u64>)>` to push the
  user-facing inconvenience to extremes.

- Demonstrate: Uncomment the `PhantomData` import, and implement the following:

  ```rust,compile_fail
  pub struct ChatId<T> {
      id: u64,
      tag: PhantomData<T>,
  }
  ```

- `PhantomData<T>` is a zero-sized type with a type parameter. We can construct
  values of it like other ZSTs with
  `let phantom: PhantomData<UserTag> = PhantomData;` or with the
  `PhantomData::default()` implementation.

- `PhantomData` can be used as part of the Typestate pattern to have data with
  the same structure i.e. `TaggedData<Start>` have methods or trait
  implementations that `TaggedData<End>` doesn't.

## More to Explore

- What have we lost behavior-wise with this change in implementation?

  Answer: Different ID types implementing multiple traits depending on their
  permissions.

  This behavior means we can use an admin ID for methods that only require user
  permissions.

  We can implement this by having traits for the different permission levels,
  then implementing methods on `ChatId<T>` depending on what traits `T`
  implements.

</details>

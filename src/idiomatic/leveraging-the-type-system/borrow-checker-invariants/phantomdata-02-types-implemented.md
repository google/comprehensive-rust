---
minutes: 10
---

# PhantomData 2/4: Type-level tagging

Let's solve the problem from the previous slide by adding a type parameter.

<!-- dprint-ignore-start -->
```rust,editable
// use std::marker::PhantomData;

pub struct ChatId<T> { id: u64, tag: T }

pub struct UserTag;
pub struct AdminTag;

pub trait ChatUser {/* ... */}
pub trait ChatAdmin {/* ... */}

impl ChatUser for UserTag {/* ... */}
impl ChatUser for AdminTag {/* ... */} // Admins are users
impl ChatAdmin for AdminTag {/* ... */}

// impl <T> Debug for UserTag<T> {/* ... */}
// impl <T> PartialEq for UserTag<T> {/* ... */}
// impl <T> Eq for UserTag<T> {/* ... */}
// And so on ...

impl <T: ChatUser> ChatId<T> {/* All functionality for users and above */}
impl <T: ChatAdmin> ChatId<T> {/* All functionality for only admins */}

fn main() {}
```
<!-- dprint-ignore-end -->

<details>

- Here we're using a type parameter and gating permissions behind "tag" types
  that implement different permission traits.

  Tag types, or marker types, are zero-sized types that have some semantic
  meaning to users and API designers.

- Ask: What issues does having it be an actual instance of that type pose?

  Answer: If it's not a zero-sized type (like `()` or `struct MyTag;`), then
  we're allocating more memory than we need to when all we care for is type
  information that is only relevant at compile-time.

- Demonstrate: remove the `tag` value entirely, then compile!

  This won't compile, as there's an unused (phantom) type parameter.

  This is where `PhantomData` comes in!

- Demonstrate: Uncomment the `PhantomData` import, and make `ChatId<T>` the
  following:

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

  Demonstrate: implement `From<u64>` for `ChatId<T>`, emphasizing the
  construction of `PhantomData`

  ```rust,compile_fail
  impl<T> From<u64> for ChatId<T> {
      fn from(value: u64) -> Self {
          ChatId {
              id: value,
              // Or `PhantomData::default()`
              tag: PhantomData,
          }
      }
  }
  ```

- `PhantomData` can be used as part of the Typestate pattern to have data with
  the same structure but different methods, e.g., have `TaggedData<Start>`
  implement methods or trait implementations that `TaggedData<End>` doesn't.

</details>

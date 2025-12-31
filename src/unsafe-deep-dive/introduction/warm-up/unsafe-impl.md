---
minutes: 4
---

# Implementing an unsafe trait

```rust,editable
pub struct LogicalClock {
    inner: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

// ...

impl Send for LogicalClock {}
impl Sync for LogicalClock {}
```

<details>

“Before we take a look at the code, we should double check that everyone knows
what a trait is. Is anyone able to explain traits for the rest of the class?

- “Traits are often described as a way to create shared behavior. Thinking about
  traits as shared behavior focuses on the syntax of methods and their
  signatures.
- “There’s also a deeper way to think of traits: as sets of requirements. This
  emphasizes the shared semantics of the implementing types.

“Can anyone explain what the `Send` and `Sync` traits are? They have

- If no
  - “Send and Sync relate to concurrency. There are many details, but broadly
    speaking. Send types are able to be shared between threads by value. Sync
    traits are able to be shared by reference.
  - There are many rules to follow to ensure that it’s safe to share data across
    thread boundaries. Those rules cannot be checked by the compiler, and
    therefore .
  - Arc implements Send and Sync, therefore it’s safe for our clock to as well.
  - It may be useful to point out that the word atomic has “indivisible” or
    “whole” from Ancient Greek, rather than the contemporary English sense of
    “tiny particle”.


Transition to next slide.

</details>

---
minutes: 4
---

# Implementing an unsafe trait

```rust,editable,ignore
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

“Can anyone explain what the `Send` and `Sync` traits are?

- If no
  - “Send and Sync relate to concurrency. There are many details, but broadly
    speaking, Send types can be shared between threads by value. Sync types must
    be shared by reference.
  - There are many rules to follow to ensure that it’s safe to share data across
    thread boundaries. Those rules cannot be checked by the compiler, and
    therefore the code author must take responsibility for upholding them.
  - Arc implements Send and Sync, therefore it’s safe for our clock to as well.
  - It may be useful to point out that the word _atomic_ has the meaning of
    “indivisible” or “whole” from Ancient Greek, rather than the contemporary
    English sense of “tiny particle”.

</details>

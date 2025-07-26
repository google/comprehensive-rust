---
course: Idiomatic Rust
session: Morning
target_minutes: 180
---

# Welcome to Idiomatic Rust

[Rust Fundamentals](../welcome-day-1.md) introduced Rust syntax and core
concepts. We now want to go one step further: how do you use Rust _effectively_
in your projects? What does _idiomatic_ Rust look like?

This course is opinionated: we will nudge you towards some patterns, and away
from others. Nonetheless, we do recognize that some projects may have different
needs. We always provide the necessary information to help you make informed
decisions within the context and constraints of your own projects.

> ⚠️ This course is under **active development**.
>
> The material may change frequently and there might be errors that have not yet
> been spotted. Nonetheless, we encourage you to browse through and provide
> early feedback!

## Schedule

{{%session outline}}

<details name="Course outline">

<!-- TODO: Remove this `details` section once the course material is finalized -->

The course will cover the topics listed below. Each topic may be covered in one
or more slides, depending on its complexity and relevance.

## Target Audience

Engineers with at least 2-3 years of coding experience in C, C++11 or newer,
Java 7 or newer, Python 2 or 3, Go or any other similar imperative programming
language. We have no expectation of experience with more modern or feature-rich
languages like Swift, Kotlin, C#, or TypeScript.

### Foundations of API design

- Golden rule: prioritize clarity and readability at the callsite. People will
  spend much more time reading the call sites than declarations of the functions
  being called.
- Make your API predictable
  - Follow naming conventions (case conventions, prefer vocabulary precedented
    in the standard library - e.g., methods should be called "push" not
    "push_back", "is_empty" not "empty" etc.)
  - Know the vocabulary types and traits in the standard library, and use them
    in your APIs. If something feels like a basic type/algorithm, check in the
    standard library first.
  - Use well-established API design patterns that we will discuss later in this
    class (e.g., newtype, owned/view type pairs, error handling)
- Write meaningful and effective doc comments (e.g., don't merely repeat the
  method name with spaces instead of underscores, don't repeat the same
  information just to fill out every markdown tag, provide usage examples)

### Leveraging the type system

- Short recap on enums, structs and type aliases
- Newtype pattern and encapsulation: parse, don't validate
- Extension traits: avoid the newtype pattern when you want to provide
  additional behaviour
- RAII, scope guards and drop bombs: using `Drop` to clean up resources, trigger
  actions or enforce invariants
- "Token" types: force users to prove they've performed a specific action
- The typestate pattern: enforce correct state transitions at compile-time
- Using the borrow checker to enforce invariants that have nothing to do with
  memory ownership
  - OwnedFd/BorrowedFd in the standard library
  - [Branded types](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf)

### Don't fight the borrow checker

- "Owned" types and "view" types: `&str` and `String`, `Path` and `PathBuf`,
  etc.
- Don't hide ownership requirements: avoid hidden `.clone()`, learn to love
  `Cow`
- Split types along ownership boundaries
- Structure your ownership hierarchy like a tree
- Strategies to manage circular dependencies: reference counting, using indexes
  instead of references
- Interior mutability (Cell, RefCell)
- Working with lifetime parameters on user-defined data types

### Polymorphism in Rust

- A quick refresher on traits and generic functions
- Rust has no inheritance: what are the implications?
  - Using enums for polymorphism
  - Using traits for polymorphism
  - Using composition
  - How do I pick the most appropriate pattern?
- Working with generics
  - Generic type parameter in a function or trait object as an argument?
  - Trait bounds don't have to refer to the generic parameter
  - Type parameters in traits: should it be a generic parameter or an associated
    type?
- Macros: a valuable tool to DRY up code when traits are not enough (or too
  complex)

### Error Handling

- What is the purpose of errors? Recovery vs. reporting.
- Result vs. Option
- Designing good errors:
  - Determine the error scope.
  - Capture additional context as the error flows upwards, crossing scope
    boundaries.
  - Leverage the `Error` trait to keep track of the full error chain.
  - Leverage `thiserror` to reduce boilerplate when defining error types.
  - `anyhow`
- Distinguish fatal errors from recoverable errors using
  `Result<Result<T, RecoverableError>, FatalError>`.

</details>

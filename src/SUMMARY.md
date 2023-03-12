# Summary

[Welcome to Comprehensive Rust 🦀](welcome.md)
- [Running the Course](running-the-course.md)
  - [Course Structure](running-the-course/course-structure.md)
  - [Day 4](running-the-course/day-4.md)
  - [Keyboard Shortcuts](running-the-course/keyboard-shortcuts.md)
  - [Translations](running-the-course/translations.md)
- [Using Cargo](cargo.md)
  - [Rust Ecosystem](cargo/rust-ecosystem.md)
  - [Code Samples](cargo/code-samples.md)
  - [Running Cargo Locally](cargo/running-locally.md)


# Day 1: Morning

----

- [Welcome](welcome-day-1.md)
  - [What is Rust?](welcome-day-1/what-is-rust.md)
- [Hello World!](hello-world.md)
  - [Small Example](hello-world/small-example.md)
- [Why Rust?](why-rust.md)
  - [Compile Time Guarantees](why-rust/compile-time.md)
  - [Runtime Guarantees](why-rust/runtime.md)
  - [Modern Features](why-rust/modern.md)
- [Basic Syntax](basic-syntax.md)
  - [Scalar Types](basic-syntax/scalar-types.md)
  - [Compound Types](basic-syntax/compound-types.md)
  - [References](basic-syntax/references.md)
    - [Dangling References](basic-syntax/references-dangling.md)
  - [Slices](basic-syntax/slices.md)
    - [String vs str](basic-syntax/string-slices.md)
  - [Functions](basic-syntax/functions.md)
    - [Methods](basic-syntax/methods.md)
    - [Overloading](basic-syntax/functions-interlude.md)
- [Exercises](exercises/day-1/morning.md)
  - [Implicit Conversions](exercises/day-1/implicit-conversions.md)
  - [Arrays and for Loops](exercises/day-1/for-loops.md)

# Day 1: Afternoon

- [Variables](basic-syntax/variables.md)
  - [Type Inference](basic-syntax/type-inference.md)
  - [static & const](basic-syntax/static-and-const.md))
  - [Scopes and Shadowing](basic-syntax/scopes-shadowing.md)
- [Memory Management](memory-management.md)
  - [Stack vs Heap](memory-management/stack-vs-heap.md)
  - [Stack Memory](memory-management/stack.md)
  - [Manual Memory Management](memory-management/manual.md)
  - [Scope-Based Memory Management](memory-management/scope-based.md)
  - [Garbage Collection](memory-management/garbage-collection.md)
  - [Rust Memory Management](memory-management/rust.md)
  - [Comparison](memory-management/comparison.md)
- [Ownership](ownership.md)
  - [Move Semantics](ownership/move-semantics.md)
  - [Moved Strings in Rust](ownership/moved-strings-rust.md)
    - [Double Frees in Modern C++](ownership/double-free-modern-cpp.md)
  - [Moves in Function Calls](ownership/moves-function-calls.md)
  - [Copying and Cloning](ownership/copy-clone.md)
  - [Borrowing](ownership/borrowing.md)
    - [Shared and Unique Borrows](ownership/shared-unique-borrows.md)
  - [Lifetimes](ownership/lifetimes.md)
  - [Lifetimes in Function Calls](ownership/lifetimes-function-calls.md)
  - [Lifetimes in Data Structures](ownership/lifetimes-data-structures.md)
- [Exercises](exercises/day-1/afternoon.md)
  - [Designing a Library](exercises/day-1/book-library.md)
  - [Iterators and Ownership](exercises/day-1/iterators-and-ownership.md)


# Day 2: Morning

----

- [Welcome](welcome-day-2.md)
- [Structs](structs.md)
  - [Tuple Structs](structs/tuple-structs.md)
  - [Field Shorthand Syntax](structs/field-shorthand.md)
- [Enums](enums.md)
  - [Variant Payloads](enums/variant-payloads.md)
  - [Enum Sizes](enums/sizes.md)
- [Methods](methods.md)
  - [Method Receiver](methods/receiver.md)
  - [Example](methods/example.md)
- [Pattern Matching](pattern-matching.md)
  - [Destructuring Enums](pattern-matching/destructuring-enums.md)
  - [Destructuring Structs](pattern-matching/destructuring-structs.md)
  - [Destructuring Arrays](pattern-matching/destructuring-arrays.md)
  - [Match Guards](pattern-matching/match-guards.md)
- [Exercises](exercises/day-2/morning.md)
  - [Health Statistics](exercises/day-2/health-statistics.md)
  - [Points and Polygons](exercises/day-2/points-polygons.md)

# Day 2: Afternoon

- [Control Flow](control-flow.md)
  - [Blocks](control-flow/blocks.md)
  - [if expressions](control-flow/if-expressions.md)
  - [if let expressions](control-flow/if-let-expressions.md)
  - [while expressions](control-flow/while-expressions.md)
  - [while let expressions](control-flow/while-let-expressions.md)
  - [for expressions](control-flow/for-expressions.md)
  - [loop expressions](control-flow/loop-expressions.md)
  - [match expressions](control-flow/match-expressions.md)
  - [break & continue](control-flow/break-continue.md)
- [Standard Library](std.md)
  - [Option and Result](std/option-result.md)
  - [String](std/string.md)
  - [Vec](std/vec.md)
  - [HashMap](std/hashmap.md)
  - [Box](std/box.md)
    - [Recursive Data Types](std/box-recursive.md)
    - [Niche Optimization](std/box-niche.md)
  - [Rc](std/rc.md)
- [Modules](modules.md)
  - [Visibility](modules/visibility.md)
  - [Paths](modules/paths.md)
  - [Filesystem Hierarchy](modules/filesystem.md)
- [Exercises](exercises/day-2/afternoon.md)
  - [Luhn Algorithm](exercises/day-2/luhn.md)
  - [Strings and Iterators](exercises/day-2/strings-iterators.md)


# Day 3: Morning

----

- [Welcome](welcome-day-3.md)
- [Traits](traits.md)
  - [Deriving Traits](traits/deriving-traits.md)
  - [Default Methods](traits/default-methods.md)
  - [Important Traits](traits/important-traits.md)
    - [Iterator](traits/iterator.md)
    - [FromIterator](traits/from-iterator.md)
    - [From and Into](traits/from-into.md)
    - [Read and Write](traits/read-write.md)
    - [Add, Mul, ...](traits/operators.md)
    - [Drop](traits/drop.md)
    - [Default](traits/default.md)
- [Generics](generics.md)
  - [Generic Data Types](generics/data-types.md)
  - [Generic Methods](generics/methods.md)
  - [Trait Bounds](generics/trait-bounds.md)
  - [impl Trait](generics/impl-trait.md)
  - [Closures](generics/closures.md)
  - [Monomorphization](generics/monomorphization.md)
  - [Trait Objects](generics/trait-objects.md)
- [Exercises](exercises/day-3/morning.md)
  - [A Simple GUI Library](exercises/day-3/simple-gui.md)

# Day 3: Afternoon

- [Error Handling](error-handling.md)
  - [Panics](error-handling/panics.md)
    - [Catching Stack Unwinding](error-handling/panic-unwind.md)
  - [Structured Error Handling](error-handling/result.md)
  - [Propagating Errors with ?](error-handling/try-operator.md)
    - [Converting Error Types](error-handling/converting-error-types.md)
      - [Example](error-handling/converting-error-types-example.md)
    - [Deriving Error Enums](error-handling/deriving-error-enums.md)
    - [Dynamic Error Types](error-handling/dynamic-errors.md)
    - [Adding Context to Errors](error-handling/error-contexts.md)
- [Testing](testing.md)
  - [Unit Tests](testing/unit-tests.md)
  - [Test Modules](testing/test-modules.md)
  - [Documentation Tests](testing/doc-tests.md)
  - [Integration Tests](testing/integration-tests.md)
- [Unsafe Rust](unsafe.md)
  - [Dereferencing Raw Pointers](unsafe/raw-pointers.md)
  - [Mutable Static Variables](unsafe/mutable-static-variables.md)
  - [Unions](unsafe/unions.md)
  - [Calling Unsafe Functions](unsafe/calling-unsafe-functions.md)
    - [Writing Unsafe Functions](unsafe/writing-unsafe-functions.md)
    - [Extern Functions](unsafe/extern-functions.md)
  - [Implementing Unsafe Traits](unsafe/unsafe-traits.md)
- [Exercises](exercises/day-3/afternoon.md)
  - [Safe FFI Wrapper](exercises/day-3/safe-ffi-wrapper.md)


# Day 4: Morning

----

- [Welcome](welcome-day-4.md)
- [Concurrency](concurrency.md)
  - [Threads](concurrency/threads.md)
  - [Scoped Threads](concurrency/scoped-threads.md)
  - [Channels](concurrency/channels.md)
    - [Unbounded Channels](concurrency/channels/unbounded.md)
    - [Bounded Channels](concurrency/channels/bounded.md)
  - [Shared State](concurrency/shared_state.md)
    - [Arc](concurrency/shared_state/arc.md)
    - [Mutex](concurrency/shared_state/mutex.md)
    - [Example](concurrency/shared_state/example.md)
  - [Send and Sync](concurrency/send-sync.md)
    - [Send](concurrency/send-sync/send.md)
    - [Sync](concurrency/send-sync/sync.md)
    - [Examples](concurrency/send-sync/examples.md)
- [Exercises](exercises/day-4/morning.md)
  - [Dining Philosophers](exercises/day-4/dining-philosophers.md)
  - [Multi-threaded Link Checker](exercises/day-4/link-checker.md)

# Day 4: Afternoon (Android)

----

- [Android](android.md)
  - [Setup](android/setup.md)
  - [Build Rules](android/build-rules.md)
    - [Binary](android/build-rules/binary.md)
    - [Library](android/build-rules/library.md)
  - [AIDL](android/aidl.md)
    - [Interface](android/aidl/interface.md)
    - [Implementation](android/aidl/implementation.md)
    - [Server](android/aidl/server.md)
    - [Deploy](android/aidl/deploy.md)
    - [Client](android/aidl/client.md)
    - [Changing API](android/aidl/changing.md)
  - [Logging](android/logging.md)
  - [Interoperability](android/interoperability.md)
    - [With C](android/interoperability/with-c.md)
      - [Calling C with Bindgen](android/interoperability/with-c/bindgen.md)
      - [Calling Rust from C](android/interoperability/with-c/rust.md)
    - [With C++](android/interoperability/cpp.md))
    - [With Java](android/interoperability/java.md)
- [Exercises](exercises/day-4/android.md)

# Final Words

- [Thanks!](thanks.md)
- [Other Resources](other-resources.md)
- [Credits](credits.md)

----

# Solutions

----

- [Solutions](exercises/solutions.md)
  - [Day 1 Morning](exercises/day-1/solutions-morning.md)
  - [Day 1 Afternoon](exercises/day-1/solutions-afternoon.md)
  - [Day 2 Morning](exercises/day-2/solutions-morning.md)
  - [Day 2 Afternoon](exercises/day-2/solutions-afternoon.md)
  - [Day 3 Morning](exercises/day-3/solutions-morning.md)
  - [Day 3 Afternoon](exercises/day-3/solutions-afternoon.md)
  - [Day 4 Morning](exercises/day-4/solutions-morning.md)

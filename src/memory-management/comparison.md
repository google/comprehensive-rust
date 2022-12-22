# Comparison

Here is a rough comparison of the memory management techniques.

## Pros of Different Memory Management Techniques

* Manual like C:
  * No runtime overhead.
* Automatic like Java:
  * Fully automatic.
  * Safe and correct.
* Scope-based like C++:
  * Partially automatic.
  * No runtime overhead.
* Compiler-enforced scope-based like Rust:
  * Enforced by compiler.
  * No runtime overhead.
  * Safe and correct.

## Cons of Different Memory Management Techniques

* Manual like C:
  * Use-after-free.
  * Double-frees.
  * Memory leaks.
* Automatic like Java:
  * Garbage collection pauses.
  * Destructors delays.
* Scope-based like C++:
  * Complex, opt-in by programmer.
  * Potential for use-after-free.
* Compiler-enforced and scope-based like Rust:
  * Some upfront complexity.
  * Can reject valid programs.

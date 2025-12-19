---
minutes: 3
---

# Unsafe keyword shifts responsibility

|             | Is Memory Safe? | Responsibility for Memory Safety |
| :---------- | :-------------: | :------------------------------- |
| Safe Rust   |       Yes       | Compiler                         |
| Unsafe Rust |       Yes       | Programmer                       |

<details>

Who has responsibility for memory safety?

- Safe Rust → compiler
- Unsafe Rust → programmer

“While writing safe Rust, you cannot create memory safety problems. The compiler
will ensure that a program with mistakes will not build.”

“The `unsafe` keyword shifts responsibility for maintaining memory safety from
the compiler to programmers. It signals that there are preconditions that must
be satisfied.

“To uphold that responsibility, programmers must ensure that they've understood
what the preconditions are and that they code will always satisfy them.

“Throughout this course, we'll use the term _safety preconditions_ to describe
this situation.”

</details>

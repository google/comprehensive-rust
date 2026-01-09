---
minutes: 3
---

# C++ ↔ C

| Concern           | C              | C++                                               |
| ----------------- | -------------- | ------------------------------------------------- |
| **Overloading**   | Manual/ad-hoc  | Automatic                                         |
| **Exceptions**    | -              | Stack unwinding                                   |
| **Destructors**   | Manual cleanup | Automatic via destructors (RAII)                  |
| **Non-POD types** | -              | Objects with constructors, vtables, virtual bases |
| **Templates**     | -              | Compile-time code generation                      |

<details>

C++ includes a number of features that don’t exist in C with an FFI impact:

Overloading: overloads become impossible to express because of name mangling

Exceptions: must catch exceptions at the FFI boundary and convert as escaping
exceptions in `extern "C"` functions is undefined behavior

Destructors: C callers won't run destructors; must expose explicit `*_destroy()`
functions

Non-POD types: Must use opaque pointers across the FFI boundary as pass by value
does not make sense

Templates: Cannot expose directly; must instantiate explicitly and wrap each
specialization

</details>

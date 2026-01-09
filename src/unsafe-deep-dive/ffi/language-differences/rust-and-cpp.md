---
minutes: 3
---

# Rust ↔ C++

| Concern                    | Rust                                     | C++                                                             |
| -------------------------- | ---------------------------------------- | --------------------------------------------------------------- |
| **Trivial relocatability** | All moves are `memcpy`                   | Self-referential types, move constructors can have side effects |
| **Destruction safety**     | `Drop::drop()` on original location only | Destructor may run on moved-from objects                        |
| **Exception safety**       | Panics (abort or unwind)                 | Exceptions (unwind)                                             |
| **ABI stability**          | Explicitly unstable                      | Vendor-specific                                                 |

<details>

Even if it were possible to avoid interop via C, there are still some areas of
the languages that impact FFI:

_Trivial relocatability_

Cannot safely move C++ objects on Rust side; must pin or keep in C++ heap.

In Rust, object movement, which occurs during assignment or by being passed by
value, always copies values bit by bit.

C++ allows users to define their own semantics by allowing them to overload the
assignment operator and create move and copy constructors.

This impacts interop because self-referential types become natural in
high-performance C++. Custom constructors can uphold safety invariants even when
the object moves its position in memory.

Objects with the same semantics are impossible to define in Rust.

_Destruction safety_

Moved-from C++ object semantics don't map; must prevent Rust from "moving" C++
types

_Exception safety_

Neither can cross into the other safely; both must catch at boundary

</details>

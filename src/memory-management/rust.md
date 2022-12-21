# Memory Management in Rust

Memory management in Rust is a mix:

* Safe and correct like Java, but without a garbage collector.
* Scope-based like C++, but the compiler enforces full adherence.
* Has no runtime overhead like in C and C++.

It achieves this by modeling _ownership_ explicitly.

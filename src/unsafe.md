# Unsafe Rust

The Rust language has two parts:

* **Safe Rust:** memory safe, no undefined behavior possible.
* **Unsafe Rust:** can trigger undefined behavior if preconditions are violated.

We will be seeing mostly safe Rust in this course, but it's important to know
what unsafe Rust is.

Unsafe Rust gives you access to five new capabilities:

* Dereference raw pointers.
* Access or modify mutable static variables.
* Access `union` fields.
* Call `unsafe` functions, including `extern` functions
* Implement `unsafe` traits.


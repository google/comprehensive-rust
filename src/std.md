# Standard Library

Rust comes with a standard library which helps establish a set of common types
used by Rust library and programs. This way, two libraries can work together
smoothly because they both use the same `String` type.

The common vocabulary types include:

* [`Option` and `Result`](std/option-result.md) types: used for optional values
  and [error handling](error-handling.md).

* [`String`](std/string.md): the default string type used for owned data.

* [`Vec`](std/vec.md): a standard extensible vector.

* [`HashMap`](std/hashmap.md): a hash map type with a configurable hashing
  algorithm.

* [`Box`](std/box.md): an owned pointer for heap-allocated data.

* [`Rc`](std/rc.md): a shared reference-counted pointer for heap-allocated data.

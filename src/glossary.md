# Glossary

The following is a glossary which aims to give a short definition of many Rust
terms. For translations, this also serves to connect the term back to the
English original.

<style>
h1#glossary ~ ul {
    list-style: none;
    padding-inline-start: 0;
}

h1#glossary ~ ul > li {
    /* Simplify with "text-indent: 2em hanging" when supported:
       https://caniuse.com/mdn-css_properties_text-indent_hanging */
    padding-left: 2em;
    text-indent: -2em;
}

h1#glossary ~ ul > li:first-line {
    font-weight: bold;
}
</style>

<!--
Translators: please add the English term in italic after your translated term.
Also, please keep the hard line breaks to ensure a nice formatting.
-->

- allocate:\
  Dynamic memory allocation on [the heap](memory-management/stack-vs-heap.md).
- argument:\
- Bare-metal Rust:\
  Low-level Rust development, often deployed to a system without an operating
  system. See [Bare-metal Rust](bare-metal.md).
- block:\
  See [Blocks](control-flow/blocks.md) and _scope_.
- borrow:\
  See [Borrowing](ownership/borrowing.md).
- borrow checker:\
  The part of the Rust compiler which checks that all borrows are valid.
- brace:\
  `{` and `}`. Also called _curly brace_, they delimit _blocks_.
- build:\
- call:\
- channel:\
  Used to safely pass messages [between threads](concurrency/channels.md).
- Comprehensive Rust 🦀:\
  The courses here are jointly called Comprehensive Rust 🦀.
- concurrency:\
- Concurrency in Rust:\
  See [Concurrency in Rust](concurrency.md).
- constant:\
- control flow:\
- crash:\
- enumeration:\
- error:\
- error handling:\
- exercise:\
- function:\
- garbage collector:\
- generics:\
- immutable:\
- integration test:\
- keyword:\
- library:\
- macro:\
  Rust macros can be recognized by a `!` in the name. Macros are used
  when normal functions are not enough. A typical example is `format!`,
  which takes a variable number of arguments, which isn't supported by
  Rust functions.
- `main` function:\
  Rust programs start executing with the `main` function.
- match:\
- memory leak:\
- method:\
- module:\
- move:\
- mutable:\
- ownership:\
- panic:\
- parameter:\
- pattern:\
- payload:\
- program:\
- programming language:\
- receiver:\
- reference counting:\
- return:\
- Rust:\
- Rust Fundamentals:\
  Days 1 to 3 of this course.
- Rust in Android:\
  See [Rust in Android](android.md).
- safe:\
- scope:\
- standard library:\
- static:\
- string:\
  A data type storing textual data. See
  [`String` vs `str`](basic-syntax/string-slices.html) for more.
- struct:\
- test:\
- thread:\
- thread safety:\
- trait:\
- type:\
- type inference:\
- undefined behavior:\
- union:\
- unit test:\
  Rust comes with built-in support for running small unit tests and larger
  integration tests. See [Unit Tests](testing/unit-tests.html).
- unsafe:\
  The subset of Rust which allows you to trigger _undefined behavior_.
  See [Unsafe Rust](unsafe.html).
- variable:\
  A memory location storing data. Variables are valid in a _scope_.

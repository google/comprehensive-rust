# Welcome to Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

This is a three day Rust course developed by the Android team. The course covers
the full spectrum of Rust, from basic syntax to advanced topics like generics
and error handling. It also includes Android-specific content on the last day.

The goal of the course is to teach you Rust. We assume you don't know anything
about Rust and hope to:

* Give you a comprehensive understanding of the Rust syntax and language.
* Enable you to modify existing programs and write new programs in Rust.
* Show you common Rust idioms.

The first three days show you the fundamentals of Rust. Following this, you're
invited to dive into one or more spezialized topics:

* [Android](android.md): a half-day course on using Rust for Android platform
  development (AOSP). This includes interoperability wtih C, C++, and Java.
* [Bare-metal](bare-metal.md): a full day class on using Rust for bare-metal
  (embedded) development. Both microcontrollers and application processors are
  covered.
* [Concurrency](concurrency.md): a full day class on concurrency in Rust. We
  cover both classical concurrency (preemptively scheduling using threads and
  mutextes) and async/await concurrency (cooperative multitasking using
  futures).


## Non-Goals

Rust is a large language and we won't be able to cover all of it in a few days.
Some non-goals of this course are:

* Learn how to develop macros, please see [Chapter 19.5 in the Rust
  Book](https://doc.rust-lang.org/book/ch19-06-macros.html) and [Rust by
  Example](https://doc.rust-lang.org/rust-by-example/macros.html) instead.

## Assumptions

The course assumes that you already know how to program. Rust is a statically
typed language and we will sometimes make comparisons with C and C++ to better
explain or contrast the Rust approach.

If you know how to program in a dynamically typed language such as Python or
JavaScript, then you will be able to follow along just fine too.

<details>

This is an example of a _speaker note_. We will use these to add additional
information to the slides. This could be key points which the instructor should
cover as well as answers to typical questions which come up in class.

</details>

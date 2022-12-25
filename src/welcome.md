# Welcome to Comprehensive Rust 🦀

This is a four day Rust course developed by the Android team. The course covers
the full spectrum of Rust, from basic syntax to advanced topics like generics
and error handling. It also includes Android-specific content on the last day.

The goal of the course is to teach you Rust. We assume you don't know anything
about Rust and hope to:

* Give you a comprehensive understanding of the Rust syntax and language.
* Enable you to modify existing programs and write new programs in Rust.
* Show you common Rust idioms.

On Day 4, we will cover Android-specific things such as:

* Building Android components in Rust.
* AIDL servers and clients.
* Interoperability with C, C++, and Java.

It is important to note that this course does not cover Android **application** 
development in Rust, and that the Android-specific parts are specifically about
writing code for Android itself, the operating system. 

## Non-Goals

Rust is a large language and we won't be able to cover all of it in a few days.
Some non-goals of this course are:

* Learn how to use async Rust --- we'll only talk a bit about async Rust when
  covering traditional concurrency primitives. Please see [Asynchronous
  Programming in Rust](https://rust-lang.github.io/async-book/) instead for
  details on this topic.
* Learn how to develop macros, please see [Chapter 19.5 in the Rust
  Book](https://doc.rust-lang.org/book/ch19-06-macros.html) and [Rust by
  Example](https://doc.rust-lang.org/rust-by-example/macros.html) instead.
* Learn the details of how to write unsafe Rust. We will talk about unsafe Rust
  on Day 3, but we do not cover the subtle details. Please see [Chapter 19.1 in
  the Rust Book](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) and
  the [Rustonomicon](https://doc.rust-lang.org/nomicon/) instead.

## Assumptions

The course assumes that you already know how to program. Rust is a statically
typed language and we will sometimes make comparisons with C and C++ to better
explain or contrast the Rust approach.

If you know how to program in a dynamically typed language such as Python or
JavaScript, then you will be able to follow along just fine too.

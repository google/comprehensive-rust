# Welcome to Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

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

* Learn how to use async Rust --- we'll only mention async Rust when
  covering traditional concurrency primitives. Please see [Asynchronous
  Programming in Rust](https://rust-lang.github.io/async-book/) instead for
  details on this topic.
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

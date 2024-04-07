---
course: Concurrency
session: Morning
target_minutes: 180
---

# Welcome to Concurrency in Rust

This is one-day course about concurrency in Rust: structuring your program so it
does multiple things concurrently or in parallel.

We will cover two major parts of Rust today:

- Multi-threaded programming using threads and mutexes.
- Concurrent programming using the `async` keyword.

## Schedule

{{%session outline}}

## Target Audience

This course builds on [Rust Fundamentals](../welcome-day-1.md). To get the most
out of the class, we expect that you are familiar with the basics of Rust, as
well as concepts such as:

- [Borrowing](../borrowing.md): you should understand the difference between
  shared borrows (`&T`) and exclusive borrows (`&mut T`),
- [Generics](../generics.md): we will use a lot of
  [trait bounds](../generics/trait-bounds.md).
- [Closures](../std-traits/closures.md): make sure you understand how closures
  capture values from their environment.
- [`Rc`](../smart-pointers/rc.md): we will use a similar type for shared
  ownership.

## Class Format

The class is meant to be very interactive! Please ask questions to drive the
exploration of Rust!

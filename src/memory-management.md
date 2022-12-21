# Memory Management

Traditionally, languages have fallen into two broad categories:

* Full control via manual memory management: C, C++, Pascal, ...
* Full safety via automatic memory management at runtime: Java, Python, Go, Haskell, ...

Rust offers a new mix:

> Full control *and* safety via compile time enforcement of correct memory
> management.

It does this with an explicit ownership concept.

First, let's refresh how memory management works.

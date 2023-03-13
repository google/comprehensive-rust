# Memory Management

Traditionally, languages have fallen into two broad categories:

* Full control via manual memory management: C, C++, Pascal, ...
* Full safety via automatic memory management at runtime: Java, Python, Go, Haskell, ...

Rust offers a new mix:

> Full control *and* safety via compile time enforcement of correct memory
> management.

It does this with an explicit ownership concept.

First, let's refresh how memory management works.

<details>
  
One way to think about it, is that the compiler correctly puts all `alloc` and `free` calls in the fixed locations for you, without resorting to the garbage collection.
  
</details>

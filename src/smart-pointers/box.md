---
minutes: 10
---

# `Box<T>`

[`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) is an owned pointer
to data on the heap:

```rust,editable
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```

```bob
 Stack                     Heap
.- - - - - - -.     .- - - - - - -.
:             :     :             :
:    five     :     :             :
:   +-----+   :     :   +-----+   :
:   | o---|---+-----+-->|  5  |   :
:   +-----+   :     :   +-----+   :
:             :     :             :
:             :     :             :
`- - - - - - -'     `- - - - - - -'
```

`Box<T>` implements `Deref<Target = T>`, which means that you can
[call methods
from `T` directly on a `Box<T>`](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion).

Recursive data types or data types with dynamic sizes need to use a `Box`:

```rust,editable
#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn main() {
    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

```bob
 Stack                           Heap
.- - - - - - - - - - - - - - .     .- - - - - - - - - - - - - - - - - - - - - - - - -.
:                            :     :                                                 :
:    list                    :     :                                                 :
:   +---------+----+----+    :     :    +---------+----+----+    +------+----+----+  :
:   | Element | 1  | o--+----+-----+--->| Element | 2  | o--+--->| Nil  | // | // |  :
:   +---------+----+----+    :     :    +---------+----+----+    +------+----+----+  :
:                            :     :                                                 :
:                            :     :                                                 :
'- - - - - - - - - - - - - - '     '- - - - - - - - - - - - - - - - - - - - - - - - -'
```

<details>

- `Box` is like `std::unique_ptr` in C++, except that it's guaranteed to be not
  null.
- A `Box` can be useful when you:
  - have a type whose size that can't be known at compile time, but the Rust
    compiler wants to know an exact size.
  - want to transfer ownership of a large amount of data. To avoid copying large
    amounts of data on the stack, instead store the data on the heap in a `Box`
    so only the pointer is moved.

- If `Box` was not used and we attempted to embed a `List` directly into the
  `List`, the compiler would not compute a fixed size of the struct in memory
  (`List` would be of infinite size).

- `Box` solves this problem as it has the same size as a regular pointer and
  just points at the next element of the `List` in the heap.

- Remove the `Box` in the List definition and show the compiler error.
  "Recursive with indirection" is a hint you might want to use a Box or
  reference of some kind, instead of storing a value directly.

# More to Explore

## Niche Optimization

```rust,editable
#[derive(Debug)]
enum List<T> {
    Element(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

A `Box` cannot be empty, so the pointer is always valid and non-`null`. This
allows the compiler to optimize the memory layout:

```bob
 Stack                           Heap
.- - - - - - - - - - - - - - .     .- - - - - - - - - - - - - -.
:                            :     :                           :
:    list                    :     :                           :
:   +---------+----+----+    :     :    +---------+----+----+  :
:   | Element | 1  | o--+----+-----+--->| Element | 2  | // |  :
:   +---------+----+----+    :     :    +---------+----+----+  :
:                            :     :                           :
:                            :     :                           :
'- - - - - - - - - - - - - - '     '- - - - - - - - - - - - - -'
```

</details>

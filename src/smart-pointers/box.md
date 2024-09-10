---
minutes: 8
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

Recursive data types or data types with dynamic sizes cannot be stored inline
without a pointer indirection, which can be worked around using `Box`:

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
  - have a type whose size can't be known at compile time, but the Rust compiler
    wants to know an exact size.
  - want to transfer ownership of a large amount of data. To avoid copying large
    amounts of data on the stack, instead store the data on the heap in a `Box`
    so only the pointer is moved.

- If `Box` was not used and we attempted to embed a `List` directly into the
  `List`, the compiler would not be able to compute a fixed size for the struct
  in memory (the `List` would be of infinite size).

- `Box` solves this problem as it has the same size as a regular pointer and
  just points at the next element of the `List` in the heap.

- Remove the `Box` in the List definition and show the compiler error. We get
  the message "recursive without indirection", because for data recursion, we
  have to use indirection, a `Box` or reference of some kind, instead of storing
  the value directly.

# More to Explore

## Niche Optimization

Though `Box` looks like `std::unique_ptr` in C++, it cannot be empty/null. This
makes `Box` one of the types that allow the compiler to optimize storage of some
enums.

For example, `Option<Box<T>>` has the same size, as just `Box<T>`, because
compiler uses NULL-value to discriminate variants instead of using explicit tag
(["Null Pointer Optimization"](https://doc.rust-lang.org/std/option/#representation)):

```rust,editable
use std::mem::size_of_val;

struct Item(String);

fn main() {
    let just_box: Box<Item> = Box::new(Item("Just box".into()));
    let optional_box: Option<Box<Item>> =
        Some(Box::new(Item("Optional box".into())));
    let none: Option<Box<Item>> = None;

    assert_eq!(size_of_val(&just_box), size_of_val(&optional_box));
    assert_eq!(size_of_val(&just_box), size_of_val(&none));

    println!("Size of just_box: {}", size_of_val(&just_box));
    println!("Size of optional_box: {}", size_of_val(&optional_box));
    println!("Size of none: {}", size_of_val(&none));
}
```

</details>

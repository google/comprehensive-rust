---
minutes: 5
---

# Move Semantics

An assignment will transfer _ownership_ between variables:

```rust,editable
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```

- The assignment of `s1` to `s2` transfers ownership.
- When `s1` goes out of scope, nothing happens: it does not own anything.
- When `s2` goes out of scope, the string data is freed.

Before move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - -.
:                           :     :                                     :
:    s1                     :     :                                     :
:   +-----------+-------+   :     :   +----+----+----+----+----+----+   :
:   | ptr       |   o---+---+-----+-->| H  | e  | l  | l  | o  | !  |   :
:   | len       |     6 |   :     :   +----+----+----+----+----+----+   :
:   | capacity  |     6 |   :     :                                     :
:   +-----------+-------+   :     :                                     :
:                           :     `- - - - - - - - - - - - - - - - - - -'
:                           :
`- - - - - - - - - - - - - -'
```

After move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - -.
:                           :     :                                     :
:    s1 "(inaccessible)"    :     :                                     :
:   +-----------+-------+   :     :   +----+----+----+----+----+----+   :
:   | ptr       |   o---+---+--+--+-->| H  | e  | l  | l  | o  | !  |   :
:   | len       |     6 |   :  |  :   +----+----+----+----+----+----+   :
:   | capacity  |     6 |   :  |  :                                     :
:   +-----------+-------+   :  |  :                                     :
:                           :  |  `- - - - - - - - - - - - - - - - - - -'
:    s2                     :  |
:   +-----------+-------+   :  |
:   | ptr       |   o---+---+--'
:   | len       |     6 |   :
:   | capacity  |     6 |   :
:   +-----------+-------+   :
:                           :
`- - - - - - - - - - - - - -'
```

When you pass a value to a function, the value is assigned to the function
parameter. This transfers ownership:

```rust,editable
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
```

<details>

- Mention that this is the opposite of the defaults in C++, which copies by
  value unless you use `std::move` (and the move constructor is defined!).

- It is only the ownership that moves. Whether any machine code is generated to
  manipulate the data itself is a matter of optimization, and such copies are
  aggressively optimized away.

- Simple values (such as integers) can be marked `Copy` (see later slides).

- In Rust, clones are explicit (by using `clone`).

In the `say_hello` example:

- With the first call to `say_hello`, `main` gives up ownership of `name`.
  Afterwards, `name` cannot be used anymore within `main`.
- The heap memory allocated for `name` will be freed at the end of the
  `say_hello` function.
- `main` can retain ownership if it passes `name` as a reference (`&name`) and
  if `say_hello` accepts a reference as a parameter.
- Alternatively, `main` can pass a clone of `name` in the first call
  (`name.clone()`).
- Rust makes it harder than C++ to inadvertently create copies by making move
  semantics the default, and by forcing programmers to make clones explicit.

# More to Explore

## Defensive Copies in Modern C++

Modern C++ solves this differently:

```c++
std::string s1 = "Cpp";
std::string s2 = s1;  // Duplicate the data in s1.
```

- The heap data from `s1` is duplicated and `s2` gets its own independent copy.
- When `s1` and `s2` go out of scope, they each free their own memory.

Before copy-assignment:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

After copy-assignment:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     :                       :
:    s2                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+-----+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

Key points:

- C++ has made a slightly different choice than Rust. Because `=` copies data,
  the string data has to be cloned. Otherwise we would get a double-free when
  either string goes out of scope.

- C++ also has [`std::move`], which is used to indicate when a value may be
  moved from. If the example had been `s2 = std::move(s1)`, no heap allocation
  would take place. After the move, `s1` would be in a valid but unspecified
  state. Unlike Rust, the programmer is allowed to keep using `s1`.

- Unlike Rust, `=` in C++ can run arbitrary code as determined by the type which
  is being copied or moved.

[`std::move`]: https://en.cppreference.com/w/cpp/utility/move

</details>

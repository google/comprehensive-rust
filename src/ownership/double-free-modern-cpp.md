# Defensive Copies in Modern C++ 

Modern C++ solves this differently:

```c++
std::string s1 = "Cpp";
std::string s2 = s1;  // Duplicate the data in s1.
```

* The heap data from `s1` is duplicated and `s2` gets its own independent copy.
* When `s1` and `s2` go out of scope, they each free their own memory.

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

<details>

Key points:

- C++ has made a slightly different choice than Rust. Because `=` copies data,
  the string data has to be cloned. Otherwise we would get a double-free when
  either string goes out of scope.

- C++ also has [`std::move`], which is used to indicate when a value may be
  moved from. If the example had been `s2 = std::move(s1)`, no heap allocation
  would take place. After the move, `s1` would be in a valid but unspecified
  state. Unlike Rust, the programmer is allowed to keep using `s1`.

- Unlike Rust, `=` in C++ can run arbitrary code as determined by the type
  which is being copied or moved.

[`std::move`]: https://en.cppreference.com/w/cpp/utility/move
 
</details>

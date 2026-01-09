# Language Interop

Ideal scenario:

```bob
╭────────────╮                                          ╭────────────╮
│            │                                          │            │
│            │ <--------------------------------------> │            │
│            │                                          │            │
╰────────────╯                                          ╰────────────╯
     Rust                                                    "C++"
```

<details>

This section of the course covers interacting with Rust and external languages
via its foreign-function interface (FFI), with a special focus on
interoperability with C++.

Ideally, users of Rust and the external language (in this case C++) could call
each others’ methods directly.

This ideal scenario is very difficult to achieve:

Different languages have different semantics and mapping between them implies
trade-offs Neither Rust nor C++ offer ABI stability[^1], making it difficult to
build from a stable foundation

[^1]: Some C++ compiler vendors provide support for ABI stability within their
    toolchain.

</details>

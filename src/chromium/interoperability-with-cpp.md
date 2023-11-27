# Interoperability with C++

The Rust community offers multiple options for C++/Rust interop, with new tools
being developed all the time. At the moment, Chromium uses a tool called "cxx".

You describe your whole language boundary in an interface definition language
(which looks a lot like Rust) and then cxx tools generate declarations for
functions and types in both Rust and C++.

<img src="../android/interoperability/cpp/overview.svg" alt="Overview diagram of cxx, showing that the same interface definition is used to create both C++ and Rust side code which then communicate via a lowest common denominator C API">

See the [CXX tutorial][1] for a full example of using this.


[1]: https://cxx.rs/tutorial.html
[2]: https://cxx.rs/bindings.html


<details>
Talk through the diagram. Explain that behind the scenes, this is doing
just the same as you previously did - but by programmatically ensuring that
the C++ and Rust sides match, cxx can ensure there aren't obvious errors
with object lifetimes, string lengths, etc. It reduces lots of fiddly
boilerplate and the resulting code feels more "natural".
</details>
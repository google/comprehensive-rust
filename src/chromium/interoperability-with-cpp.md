# Interoperability with C++

The Rust community offers multiple options for C++/Rust interop, with new tools
being developed all the time. At the moment, Chromium uses a tool called "cxx".

Behind the scenes, this uses a "lowest common denominator" C ABI, but it builds
nice abstractions on both sides.

The overall approach looks like this:

<img src="../android/interoperability/cpp/overview.svg">

See the [CXX tutorial][1] for a full example of using this.


<details>
Students may ask - why do we still need `allow_unsafe = true`?

The broad answer is that no C/C++ code is "safe" by the normal Rust standards.
Calling back and forth to C/C++ from Rust may do arbitrary things to memory, and
compromise the safety of Rust's own data layouts. Presence of _too many_
`unsafe` keywords in C/C++ interop can harm the signal-to-noise ratio of
such a keyword, and is [controversial][3], but strictly, bringing any foreign
code into a Rust binary can cause unexpected behavior from Rust's perspective.

The narrow answer lies in the diagram at the top of this page - behind the
scenes, cxx generates Rust `unsafe` and `extern "C"` functions just like
we did manually in the previous section.
</details>

[1]: https://cxx.rs/tutorial.html
[2]: https://cxx.rs/bindings.html
[3]: https://steveklabnik.com/writing/the-cxx-debate
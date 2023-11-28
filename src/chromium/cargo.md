# Using cargo for experimental tools

Subjectively,

```bob
High        ^
            |      x cargo
            |
Development |                     x "cargo --offline"
speed       |
            |                                  x  "gn/ninja"
            |                                     "rust_executable(...)" 
Low         +---------------------------------------------------->
             Low               Determinism                 High
```

`cargo` works great for pure-Rust tools, but isn't optimized for large multi-
language projects like Chromium. Chromium uses `gn` and `ninja`.

When writing a tool in Rust, your choices are:

* Use `gn` and `ninja` (using the `rust_executable` template we'll meet
  later)
* Use `cargo`, but [restrict yourself to Chromium's audited toolchain and crates][0]
* Use `cargo`, trusting a [toolchain][1] and [crates downloaded from the internet][2]

Your organization's policy, and/or common sense, may prohibit you from doing
these things.

From here on we'll be focusing on `gn` and `ninja`.

## Mini exercise

Discuss in small groups the policies within your own team and organization,
and come to a group agreement about what's an acceptable level of risk.

<details>

Explain that it might seem strange to write tools in Rust, but this is
increasingly popular across the industry --- Rust tools are quicker and work
more reliably.

Assuming folks taking the course are physically together, ask them to discuss
in small groups of 3-4 people. Then, ask each table whether they've come
to a consensus on the level of risk.

Later in the course, we'll be running an actual `cargo`-based tool, `gnrt`.

</details>

[0]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Using-cargo
[1]: https://rustup.rs/
[2]: https://crates.io/

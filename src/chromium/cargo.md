# Comparing Chromium and Cargo Ecosystems

Rust community typically uses `cargo` and libraries from [crates.io][2].
Chromium is built using `gn` and `ninja` and a curated set of dependencies.

When writing code in Rust, your choices are:

- Use `gn` and `ninja` with the help of the templates from `//build/rust/*.gni`
  (e.g. `rust_static_library` that we'll meet later). This uses Chromium's
  audited toolchain and crates.
- Use `cargo`, but
  [restrict yourself to Chromium's audited toolchain and crates][0]
- Use `cargo`, trusting a [toolchain][1] and/or
  [crates downloaded from the internet][2]

From here on we'll be focusing on `gn` and `ninja`, because this is how Rust
code can be built into the Chromium browser. At the same time, Cargo is an
important part of the Rust ecosystem and you should keep it in your toolbox.

## Mini exercise

Split into small groups and:

- Brainstorm scenarios where `cargo` may offer an advantage and assess the risk
  profile of these scenarios.
- Discuss which tools, libraries, and groups of people need to be trusted when
  using `gn` and `ninja`, offline `cargo`, etc.

<details>

Ask students to avoid peeking at the speaker notes before completing the
exercise. Assuming folks taking the course are physically together, ask them to
discuss in small groups of 3-4 people.

Notes/hints related to the first part of the exercise ("scenarios where Cargo
may offer an advantage"):

- It's fantastic that when writing a tool, or prototyping a part of Chromium,
  one has access to the rich ecosystem of crates.io libraries. There is a crate
  for almost anything and they are usually quite pleasant to use. (`clap` for
  command-line parsing, `serde` for serializing/deserializing to/from various
  formats, `itertools` for working with iterators, etc.).

  - `cargo` makes it easy to try a library (just add a single line to
    `Cargo.toml` and start writing code)
  - It may be worth comparing how CPAN helped make `perl` a popular choice. Or
    comparing with `python` + `pip`.

- Development experience is made really nice not only by core Rust tools (e.g.
  using `rustup` to switch to a different `rustc` version when testing a crate
  that needs to work on nightly, current stable, and older stable) but also by
  an ecosystem of third-party tools (e.g. Mozilla provides `cargo vet` for
  streamlining and sharing security audits; `criterion` crate gives a
  streamlined way to run benchmarks).

  - `cargo` makes it easy to add a tool via `cargo install --locked cargo-vet`.
  - It may be worth comparing with Chrome Extensions or VScode extensions.

- Broad, generic examples of projects where `cargo` may be the right choice:

  - Perhaps surprisingly, Rust is becoming increasingly popular in the industry
    for writing command line tools. The breadth and ergonomics of libraries is
    comparable to Python, while being more robust (thanks to the rich
    typesystem) and running faster (as a compiled, rather than interpreted
    language).
  - Participating in the Rust ecosystem requires using standard Rust tools like
    Cargo. Libraries that want to get external contributions, and want to be
    used outside of Chromium (e.g. in Bazel or Android/Soong build environments)
    should probably use Cargo.

- Examples of Chromium-related projects that are `cargo`-based:
  - `serde_json_lenient` (experimented with in other parts of Google which
    resulted in PRs with performance improvements)
  - Fontations libraries like `font-types`
  - `gnrt` tool (we will meet it later in the course) which depends on `clap`
    for command-line parsing and on `toml` for configuration files.
    - Disclaimer: a unique reason for using `cargo` was unavailability of `gn`
      when building and bootstrapping Rust standard library when building Rust
      toolchain.)
    - `run_gnrt.py` uses Chromium's copy of `cargo` and `rustc`. `gnrt` depends
      on third-party libraries downloaded from the internet, by `run_gnrt.py`
      asks `cargo` that only `--locked` content is allowed via `Cargo.lock`.)

Students may identify the following items as being implicitly or explicitly
trusted:

- `rustc` (the Rust compiler) which in turn depends on the LLVM libraries, the
  Clang compiler, the `rustc` sources (fetched from GitHub, reviewed by Rust
  compiler team), binary Rust compiler downloaded for bootstrapping
- `rustup` (it may be worth pointing out that `rustup` is developed under the
  umbrella of the https://github.com/rust-lang/ organization - same as `rustc`)
- `cargo`, `rustfmt`, etc.
- Various internal infrastructure (bots that build `rustc`, system for
  distributing the prebuilt toolchain to Chromium engineers, etc.)
- Cargo tools like `cargo audit`, `cargo vet`, etc.
- Rust libraries vendored into `//third_party/rust` (audited by
  security@chromium.org)
- Other Rust libraries (some niche, some quite popular and commonly used)

</details>

[0]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Using-cargo
[1]: https://rustup.rs/
[2]: https://crates.io/

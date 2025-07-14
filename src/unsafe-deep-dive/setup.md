---
minutes: 2
---

# Setting Up

## Local Rust installation

You should have a Rust compiler installed that supports the 2024 edition of the
language, which is any version of rustc higher than 1.84.

```console
$ rustc --version 
rustc 1.87
```

<!--

  TODO (tim): Adding this for later while I'm here.
  TODO (tim): We should be able to avoid this by just relying on the `cc` crate

We recommend that you install the [Bazel build system](https://bazel.build/install).
This will allow you to easily compile project that combine multiple languages.

-->

## (Optional) Create a local instance of the course

Having a local version of the course material is useful in case of any
interruptions with the network and makes it easy to access it later.

```console
$ git clone --depth=1 https://github.com/google/comprehensive-rust.git
Cloning into 'comprehensive-rust'...
...
$ cd comprehensive-rust
$ cargo install-tools
...
$ cargo serve # then open http://127.0.0.1:3000/ in a browser
```

<details>

Ask everyone to confirm that everyone is able to execute `rustc` with a version
older that 1.87.

For those people who do not, tell them that we'll resolve that in the break.

</details>

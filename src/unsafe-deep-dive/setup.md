---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

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
This will allow you to easily compile projects that combine multiple languages.

-->

## (Optional) Create a local instance of the course

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
newer than 1.87.

For those people who do not, tell them that we'll resolve that in the break.

</details>

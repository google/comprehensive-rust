---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Procedural Macro Basics

To write a procedural macro, you must define it in a separate library crate
dedicated solely to macros.

### Cargo.toml configuration

```toml
[lib]
proc-macro = true
```

### Compiler Plugin Architecture

- **Build-Time Execution:** The macro crate is compiled first as a dynamic
  library. The compiler loads this library and runs the macro functions during
  the compilation of the consuming crate.
- **Dependency Isolation:** Because macro functions execute during compilation,
  any dependencies imported by your macro crate (like parsing helper libraries)
  are **build-time dependencies**, and are not compiled into the final client
  binary.

<details>

- Explain that because the macro code runs _inside_ the compiler, it cannot be
  mixed into standard library crates. If you try to define a procedural macro in
  a standard crate, the compiler will emit a compile-time error.
- Point out that this architecture guarantees that procedural macro logic can
  leverage the complete Rust standard library (file access, networking,
  multi-threading) to perform compile-time tasks, though doing so should be kept
  to a minimum to keep compiles fast and secure.

</details>

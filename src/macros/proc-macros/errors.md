---
minutes: 7
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Handling Errors

Because procedural macros execute during compilation, standard runtime error
reporting strategies do not apply. If a macro receives malformed input, it must
signal the error to the compiler.

There are two main ways to report errors in procedural macros:

### 1. Panicking

If your macro code panics, the compiler will catch the panic, halt compilation,
and print the panic message as a compiler error:

```rust,ignore
panic!("Invalid struct name")
```

This may occur by default if there are any bugs or programming errors in your
procedural macro.

### 2. Emitting `compile_error!` (Preferred)

A more friendly approach is to return a `TokenStream` containing a call to the
built-in `compile_error!` macro. This allows the compiler to highlight the exact
span where the error occurred:

```rust,ignore
// Returning an explicit compile error TokenStream:
syn::Error::new(span, "Expected a valid identifier")
    .to_compile_error() // Generates compile_error!("...")
    .into()
```

<details>

- Explain that panicking is easy but has a terrible developer experience. It
  halts the entire compilation and doesn't tell the developer where they made a
  mistake (the error is tracked to the macro definition site rather than the
  particular call site).
- Emphasize that emitting `compile_error!` is the standard, idiomatic way to
  handle user input validation. The error message is attached to a specific
  `Span` (the location of the bad token), which allows IDEs and compiler error
  messages to draw a red squiggly line directly under the offending code.

</details>

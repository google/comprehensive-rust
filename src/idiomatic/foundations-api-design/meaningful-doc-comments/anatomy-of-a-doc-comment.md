---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The Anatomy of a Doc Comment

1. A brief, one-sentence summary.
2. A more detailed explanation.
3. Special sections: code examples, panics, errors, safety preconditions.

````rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Parses a key-value pair from a string.
///
/// The input string must be in the format `key=value`. Everything before the
/// first '=' is treated as the key, and everything after is the value.
///
/// # Examples
///
/// ```
/// use my_crate::parse_key_value;
/// let (key, value) = parse_key_value("lang=rust").unwrap();
/// assert_eq!(key, "lang");
/// assert_eq!(value, "rust");
/// ```
///
/// # Panics
///
/// Panics if the input is empty.
///
/// # Errors
///
/// Returns a `ParseError::Malformed` if the string does not contain `=`.
///
/// # Safety
///
/// Triggers undefined behavior if...
unsafe fn parse_key_value(s: &str) -> Result<(String, String), ParseError>

enum ParseError {
    Empty,
    Malformed,
}
````

<details>

- Idiomatic Rust doc comments follow a conventional structure that makes them
  easier for developers to read.

- The first line of a doc comment is a single-sentence summary of the function.
  Keep it concise. `rustdoc` and other tools have a strong expectation about
  that: it is used as a short summary in module-level documentation and search
  results.

- Next, you can provide a long, multi-paragraph description of the "why" and
  "what" of the function. Use Markdown.

- Finally, you can use top-level section headers to organize your content. Doc
  comments commonly use `# Examples`, `# Panics`, `# Errors`, and `# Safety` as
  section titles. The Rust community expects to see relevant aspects of your API
  documented in these sections.

- Rust heavily focuses on safety and correctness. Documenting behavior of your
  code in case of errors is critical for writing reliable software.

- `# Panics`: If your function may panic, you must document the specific
  conditions when that might happen. Callers need to know what to avoid.

- `# Errors`: For functions returning a `Result`, this section explains what
  kind of errors can occur and under what circumstances. Callers need this
  information to write robust error handling logic.

- **Question:** Ask the class why documenting panics is so important in a
  language that prefers returning `Result`.

  - **Answer:** Panics are for unrecoverable, programming errors. A library
    should not panic unless a contract is violated by the caller. Documenting
    these contracts is essential.

- `# Safety` comments document safety preconditions on unsafe functions that
  must be satisfied, or else undefined behavior might result. They are discussed
  in detail in the Unsafe Rust deep dive.

</details>

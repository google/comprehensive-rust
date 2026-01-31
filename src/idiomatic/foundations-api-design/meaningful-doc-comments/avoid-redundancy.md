---
minutes: 15
---

# Avoiding Redundancy

Names and type signatures communicate significant information, don't repeat it
in comments!

```rust,compile_fail
// Repeats name/type information. Can omit!
/// Parses an ipv4 from a str. Returns an option for failure modes.
fn parse_ip_addr_v4(input: &str) -> Option<IpAddrV4> { ... }

// Repeats information obvious from the field name. Can omit!
struct BusinessAsset {
  /// The customer id.
  let customer_id: u64,
}

// Mentions the type name first thing, don't do this!
/// `ServerSynchronizer` is an orchestrator that sends local edits [...]
struct ServerSynchronizer { ... }

// Better! Focuses on purpose.
/// Sends local edits [...]
struct ServerSynchronizer { ... }

// Mentions the function name first thing, don't do this!
/// `sync_to_server` sends local edits [...]
fn sync_to_server(...)

// Better! Focuses on function.
/// Sends local edits [...]
fn sync_to_server(...)
```

<details>

- Motivation: Documentation that merely repeats name/signature information
  provides nothing new to the API user.

Additionally, signature information may change over time without the
documentation being updated accordingly!

- This is an understandable pattern to fall into!

  Naive approach to "always document your code," follows this advice literally
  but does not follow the intent.

  Some tools might enforce documentation coverage, this kind of documentation is
  an easy fix.

- Be aware of the purpose of different modes of documentation:

  - Library code will need to be documented in ways that understand the scope of
    what it is used for and the breadth of people who are trying to use it.

  - Application code has a more narrow purpose, it can afford to be more simple
    and direct.

- The name of an item is part of the documentation of that item.

  Similarly, the signature of a function is part of the documentation of that
  function.

  Therefore: Some aspects of the item are already covered when you start writing
  doc comments!

  Do not repeat information for the sake of an itemized list.

- Many areas of the standard library have minimal documentation because the name
  and types do give enough information.

  Rule of Thumb: What information is missing from a user's perspective? Other
  than name, signature, and irrelevant details of the implementation.

- Don't explain the basics of Rust or the standard library. Assume the reader of
  doc comments has an intermediate understanding of the language itself. Focus
  on documenting your API.

  For example, if your function returns `Result`, you don't need to explain how
  `Result` or the question mark operators work.

- If there is a complex topic involved with the functions and types you're
  documenting, signpost to a "source of truth" if one exists such as an internal
  document, a paper, a blog post etc.

- Collaborate with Students: Go through the methods in the slide and discuss
  what might be relevant to an API user.

## More to Explore

- The `#![warn(missing_docs)]` lint can be helpful for enforcing the existence
  of doc comments, but puts a large burden on developers that could lead to
  leaning onto these patterns of writing low-quality comments.

  This kind of lint should only be enabled if the people maintaining a project
  can afford to keep up with its demands, and usually only for library-style
  crates rather than application code.

</details>

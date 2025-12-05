---
minutes: 15
---

# Avoiding Redundancy

Function names and type signatures already document some information, avoid
repeating them!

```rust
// Don't do this!
/// Parses an ipv4 from a str. Returns an option for failure modes.
fn parse_ip_addr_v4(input: &str) -> Option<IpAddrV4> { ... }

// TODO: couple more of these, for the instructor to go through with students.
```

<details>
- Motivation: Documentation that repeats name/signature information provides nothing new to the API user.

- This is an understandable pattern to fall into!

  Naive approach to "always document your code," follows this advice literally
  but does not follow the intent.

  Tests might enforce documentation coverage, this kind of documentation is an
  easy fix.

- The name of a function or type is part of the documentation of that function
  or type.

  Similarly, the signature of a function is part of the documentation of that
  function.

  Therefore: aspects of the subject are already covered when you start writing
  doc comments!

- Many areas of the standard library have minimal documentation because the name
  and types do give enough information.

  Rule of Thumb: What information is missing from a user's perspective? Other
  than name, signature, and irrelevant details of the implementation.

- Don't drop down to language basics! Assume the reader of doc comments has an
  intermediate understanding of the language itself, it's the API you're working
  on that you're trying to document.

- If there is a complex topic involved with the functions and types you're
  documenting, signpost to a "source of truth" if one exists such as a blog
  post, an internal document, a paper etc.

- Collaborate with Students: Go through the methods in the slide and discuss
  what might be relevant to an API user.

</details>

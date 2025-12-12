---
minutes: 10
---

# Who are you writing for?

Colleagues, collaborators, largely-silent API users, or just yourself?

```rust
// expert writes for experts
/// Canonicalizes the MIR for the borrow checker.  
///  
/// This pass ensures that all borrows conform to the NLL-Polonius constraints  
/// before we proceed to MIR-to-LLVM-IR translation.  
pub fn canonicalize_mir(mir: &mut Mir) {
    // ...
}

// expert writes for newcomers
/// Prepares the Mid-level IR (MIR) for borrow checking.  
///  
/// The borrow checker operates on a simplified, "canonical" form of the MIR.  
/// This function performs that transformation. It is a prerequisite for the  
/// final stages of code generation.  
///  
/// For more about Rust's intermediate representations, see the  
/// [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/mir/index.html).  
pub fn canonicalize_mir(mir: &mut Mir) {
    // ...
}
```

<details>

- Background: The
  [curse of knowledge](https://en.wikipedia.org/wiki/Curse_of_knowledge) is a
  cognitive bias where experts assume that others have the same level of
  expertise and perspective.

- Motivation: Your reader does not have the same level of expertise and the same
  perspective as you. Don't write for people like yourself, write for others.

- Unintentionally writing for yourself can lead to people not understanding a
  point you're trying to make or the concept you're trying to articulate.

- Imagine a version of you, or others you've known, struggling to find practical
  information while going through documentation.

  Keep this idea of a person in mind when thinking about what areas of a
  codebase need attention for doc comments.

- Who are you writing for?

- Also imagine a version of you, or others you've known, who is struggling to
  find the important details in winding, extensive doc comments. Don't give too
  much information.

- Always ask: Is this documentation making it difficult for the API user? Are
  they able to quickly grasp what they need or find out where they could need
  it?

- Always consider: Experts also read API level documentation. Doc comments might
  not be the right place to educate your audience about the basics of your
  domain. In that case, signpost and name-drop. Divert people to long-form
  documentation.

</details>

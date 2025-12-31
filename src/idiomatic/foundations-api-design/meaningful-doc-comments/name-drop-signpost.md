---
minutes: 15
---

# Name-dropping keywords and signposting topics

```rust
/// A parsed representation of a MARC 21 record
/// [leader](//www.loc.gov/marc/bibliographic/bdleader.html).  
/// A MARC leader contains metadata that dictates how to interpret the rest  
/// of the record.
pub struct Leader {
    /// Determines the schema and the set of valid subsequent data fields.  
    ///
    /// Encoded in byte 6 of the leader.  
    pub type_of_record: char,

    /// Indicates whether to parse relationship fields, such as a "773 Host  
    /// Item Entry" for an article within a larger work.  
    ///  
    /// Encoded in byte 7 of the leader.  
    pub bibliographic_level: char,
    // ... other fields
}

/// Parses the [leader of a MARC 21 record](https://www.loc.gov/marc/bibliographic/bdleader.html).  
///  
/// The leader is encoded as a fixed-length 24-byte field, containing metadata  
/// that determines the semantic interpretation of the rest of the record.  
pub fn parse_leader(leader_bytes: &[u8; 24]) -> Result<Leader, MarcError> {
    todo!()
}

#[derive(Debug)]
pub enum MarcError {}
```

<details>

- Motivation: Readers of documentation will not be closely reading most of your
  doc comments like they would dialogue in a novel they love.

Users will most likely be skimming and scan-reading to find the part of the
documentation that is relevant to whatever problem they're trying to solve in
the moment.

Once a user has found a keyword or potential signpost that's relevant to them
they will begin to search for context surrounding what is being documented.

- Ask the class: What do you look for in documentation? Focus on the
  moment-to-moment searching for information here, not general values in
  documentation.

- Name-drop keywords close to the beginning of a paragraph.

  This aids skimming and scanning, as the first few words of a paragraph stand
  out the most.

  Skimming and scanning lets users quickly navigate a text, keeping keywords as
  close to the beginning of a paragraph as possible lets a user determine if
  they've found relevant information faster.

- Signpost, but don't over-explain.

  Users will not necessarily have the same domain expertise as an API designer.

  If a tangential, specialist term or acronym is mentioned try to bring in
  enough context such that a novice could quickly do more research.

- Signposting often happens organically, consider a networking library that
  mentions various protocols. But when it doesn't happen organically, it can be
  difficult to choose what to mention.

  Rule of thumb: API developers should be asking themselves "if a novice ran
  into what they are documenting, what sources would they look up and are there
  any red herrings they might end up following"?

  Users should be given enough information to look up subjects on their own.

- What we've already covered, predictability of an API including the naming
  conventions, is a form of signposting.

</details>

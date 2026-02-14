---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Dialog on Details

Unnecessary details can sometimes be indicative of something that does need
documentation.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Sorts a slice. Implemented using recursive quicksort.
fn sort_quickly<T: Ord>(to_sort: &mut [T]) { ... }
```

<details>

- Consider the example here, we discussed in
  [what and why, not how and where](what-why-not-how-where.md) that internal
  details are unlikely relevant to someone reading documentation.

  Here we're discussing a counterexample.

- Ask the class: Is this comment necessary for this function?

- Narrative: Playing the part of an intermediary between the class and the
  author, such as a PM, manager, etc. tell the class that the author of this
  function is pushing back.

- Ask the class: Why would an author of this kind of comment push back?

  If the class asks why the author is pushing back, do not give details yet.

- Ask the class: Why would the caller need to know the sorting algorithm in use?

- Narrative: "Come back" from a meeting with the original author, explain to the
  class that this function is application code that is called on untrusted data
  that
  [could be crafted maliciously to cause quadratic behavior during sorting](https://www.cs.dartmouth.edu/~doug/mdmspe.pdf).

- Ask the class: Now we have more detail, how should we comment this function?

  The point being implementation detail vs not depends a lot on what the public
  contract is (e.g., can you supply untrusted data or not), and this requires
  careful judgement.

  Consider if a comment is explaining that a for-loop is used (unnecessary
  detail) or if it is explaining that the algorithms used internally have known
  exploits (documentation draws attention to the wrong thing).

</details>

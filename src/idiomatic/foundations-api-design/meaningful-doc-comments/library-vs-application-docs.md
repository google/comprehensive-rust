---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Library vs application docs

You might see elaborate documentation for fundamental APIs that repeats the
names and type signatures. Stable and highly reusable code can afford this with
a positive RoI.

- Library code:
  - has a high number of users,
  - solves a whole range of related problems,
  - often has stable APIs.

- Application code is the opposite:
  - few users,
  - solves a specific problem,
  - changes often.

<details>

- You might have seen elaborate documentation that repeats code, looks at the
  same API multiple times with many examples and case studies. Context is key:
  who wrote it, for whom, and what material it is covering, and what resources
  did they have.

- Fundamental library code often has Elaborate documentation, for example, the
  standard library, highly reusable frameworks like serde and tokio. Teams
  responsible for this code often have appropriate resources to write and
  maintain elaborate documentation.

- Library code is often stable, so the community is going to extract a
  significant benefit from elaborate documentation before it needs to be
  reworked.

- Application code has the opposite traits: it has few users, solves a specific
  problem, and changes often. For application code elaborate documentation
  quickly becomes outdated and misleading. It is also difficult to extract a
  positive RoI from boilerplate docs even while they are up to date, because
  there are only a few users.

</details>

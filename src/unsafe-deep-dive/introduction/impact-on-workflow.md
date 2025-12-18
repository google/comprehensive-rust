---
minutes: 5
---

## Unsafe keyword shifts responsibility

- step 1: write code
  - check that pre-conditions are satisfied
  - document reasoning in safety comments
- step 2: code review
  - self-review → peer reviewer → unsafe Rust expert (when needed)
  - anyone can escalate the code change to an unsafe Rust expert

<details>

“The unsafe keyword places more responsibility on the programmer, therefore it
requires a stronger development workflow.

“But there's no such thing as "the programmer". Programmers work in teams.

“This class assumes a specific software development workflow where code review
is mandatory, and where the author and primary reviewer have access to an unsafe
Rust expert.”

“There are only a few unsafe Rust experts, and they are very busy, so we need to
optimally use their time.”

“The author and primary reviewer will verify simple unsafe Rust code themselves,
and punt to an unsafe expert when necessary.”

</details>

---
minutes: 5
---

# Impact on workflow

While writing code

- Verify that you understand the preconditions of any `unsafe` functions/traits
- Check that the preconditions are satisfied
- Document your reasoning in safety comments

Enhanced code review

- Self-review → peer reviewer → unsafe Rust expert (when needed)
- Escalate to a person who is comfortable with your code and reasoning

<details>

“The unsafe keyword places more responsibility on the programmer; therefore it
requires a stronger development workflow.

“This class assumes a specific software development workflow where code review
is mandatory, and where the author and primary reviewer have access to an unsafe
Rust expert.”

“The author and primary reviewer will verify simple unsafe Rust code themselves,
and punt to an unsafe expert when necessary.”

“There are only a few unsafe Rust experts, and they are very busy, so we need to
optimally use their time.”

</details>

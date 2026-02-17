<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Why Pin is difficult to use

- `Pin<Ptr>` is "just" a type defined in the standard library
- This satisfied the needs of its original audience, the creators of async
  runtimes, without needing to extending the core language
- That audience could accept some of its ergonomic downsides, as users of
  `async` would rarely interact with `Pin` directly

<details>

"You might wonder why Pin is so awkward to use. The answer is largely
historical."

"`Pin<Ptr>` offered a simpler implementation for the Rust project than
alternatives".

"Pin was designed primarily for the ~100 people in the world who write async
runtimes. The Rust team chose a simpler (for the compiler) but less ergonomic
design."

"More user-friendly proposals existed but were rejected as too complex for the
primary audience, who could handle the complexity."

</details>

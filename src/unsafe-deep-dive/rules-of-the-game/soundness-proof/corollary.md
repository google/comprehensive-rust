---
minutes: 2
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Soundness Proof (Part 2)

A sound function is one that can't trigger UB if its safety preconditions are
satisfied.

Corollary: All functions implemented in pure safe Rust are sound.

Proof:

- Safe Rust code has no safety preconditions.

- Therefore, callers of functions implemented in pure safe Rust always trivially
  satisfy the empty set of preconditions.

- Safe Rust code can't trigger UB.

QED.

<details>

- Read the corollary.

- Explain the proof.

- Translate into informal terms: all safe Rust code is nice. It does not have
  safety preconditions that the programmer has to think of, always plays by the
  rules, and never triggers UB.

</details>

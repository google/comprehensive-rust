---
minutes: 2
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Unsoundness

A sound function is one that can't trigger UB if its safety preconditions are
satisfied.

An unsound function can trigger UB even if you satisfy the documented safety
preconditions.

Unsound code is _bad_.

<details>

- Read the definition of unsound functions.

- Translate into informal terms: unsound code is not nice. No, that's an
  understatement. Unsound code is BAD. Even if you play by the documented rules,
  unsound code can still trigger UB!

- We don't want any unsound code in our repositories.

- Finding unsound code is the **primary** goal of the code review.

</details>

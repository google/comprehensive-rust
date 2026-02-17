---
minutes: 2
---

<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `Send`

> A type `T` is [`Send`][1] if it is safe to move a `T` value to another thread.

The effect of moving ownership to another thread is that _destructors_ will run
in that thread. So the question is when you can allocate a value in one thread
and deallocate it in another.

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html

<details>

As an example, a connection to the SQLite library must only be accessed from a
single thread.

</details>

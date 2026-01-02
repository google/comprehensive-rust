---
minutes: 5
---

# Language differences

```bob
╭────────────╮          ╭───╮           ╭───╮          ╭────────────╮
│            │          │   │           │   │          │            │
│            │ <----->  │   │ <~~~~~~~> │   │ <------> │            │ 
│            │          │   │           │   │          │            │
╰────────────╯          ╰───╯           ╰───╯          ╰────────────╯
    Rust                  C               C                 "C++"
```

<details>

Using C as a lowest common denominator means that lots of the richness available
to Rust and C++ is lost.

Each translation has the potential for semantic loss, runtime overhead, and
subtle bugs.

</details>

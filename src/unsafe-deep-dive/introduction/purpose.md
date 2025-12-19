---
minutes: 5
---

# Why the unsafe keyword exists

- Rust ensures safety
- But there are limits to what the compiler can do
- The unsafe keyword allows programmers to assume responsibility for Rust’s
  rules

```
┌───────────────┐   safe Rust    ┌──────────────┐
│  Programmer   │───────────────▶│   Compiler   │
└───────────────┘  (checks rules)└─────┬────────┘
                                      │ enforces safety
                                      ▼
                             Memory safety guaranteed

┌───────────────┐      unsafe     ┌──────────────┐
│  Programmer   │◀───────────────┤   Compiler   │
└──────┬────────┘  keyword shifts └──────────────┘
       │ responsibility
       ▼
Assume and uphold Rust's rules yourself
```

<details>

“A fundamental goal of Rust is to ensure memory safety.”

“But, there are limits. Some safety considerations cannot be expressed in a
programming language. Even if they could be, there are limits to what the Rust
compiler can control.”

“The `unsafe` keyword shifts the burden of upholding Rust’s rules from the
compiler to the programmer.”

“When you see the `unsafe` keyword, you are seeing responsibility shift from the
compiler to the programmer.

</details>

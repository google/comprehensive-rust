---
minutes: 1
---

# Why for learning unsafe Rust

We know that writing code without the guarantees that Rust provides ...

> “Use-after-free (UAF), integer overflows, and out of bounds (OOB) reads/writes
> comprise 90% of vulnerabilities with OOB being the most common.”
>
> --— **Jeff Vander Stoep and Chong Zang**, Google.
> "[Queue the Hardening Enhancements](https://security.googleblog.com/2019/05/queue-hardening-enhancements.html)"

... so why is `unsafe` part of the language?

## Outline

{{%segment outline}}

<details>

The `unsafe` keyword exists because there is no compiler technology available
today that makes it obsolete. Compilers cannot verify everything.

</details>

---
minutes: 5
---

> TODO: Add example

# Interop

Rust has no understanding of code generated from other languages.

<details>

Most libraries with a C ABI (which is most software libraries) provide an API
that makes heavy use of `*void` pointers. The caller is provided no type
information. Instead, users of the API are expected to read the documentation,
read the correct number of bytes from that pointer and cast those bytes to the
intended type.

As the Rust compiler can't enforce any guarantees the safety of programs that it
hasn't compiled, it must delegate that responsibility to you through the unsafe
keyword.

This isn't the only style of interoperability, however it is the method that's
needed if you want to work between Rust and some other language in a zero cost
way.

Message passing avoids unsafe, but serialization, allocation, data transfer and
parsing all take energy and time.

</details>

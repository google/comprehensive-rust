---
minutes: 5
---

# Unsafe is sometimes necessary

The Rust compiler cannot control its external environment.

<!-- mdbook-xgettext: skip -->

```bob
   specifications

      ╭──────────╮    other
      │   Rust   │    programs
      ╰──────────╯ 
                    OS
  CPU
          hardware
```

_Environment of a Rust program_

<details>

“There are some activities that _require_ unsafe.”

“A few slides ago, we saw that the Send and Sync traits require that types
comply with very strict rules. Those rules cannot be encoded into software, and
making a mistake can cause serious problems. Yet, Rust verifies that types
preserve its safety guarantees. The unsafe keyword shifts this verification
burden to the trait implementor.”

“Can anyone think of any other rules?”

- `bool` is defined as using a whole byte, but may only use a single bit
- References may not be null, i.e. may not use the all zero bit pattern

“What is meant by other programs in the diagram?”

- “Other programs’ memory. If we’re provided access to memory that’s managed by
  a program outside of Rust’s control, then Rust has no ability to confirm that
  its rules are being complied with.”

“Why is the operating system in the diagram?”

- “The operating system is another source of memory that’s external to the
  program. Until you've explicitly confirmed that the memory is correctly
  initialized, Rust will require you to use the unsafe to read from it. Memory
  that’s potentially uninitialized is very hazardous and requires special
  treatment.” “The operating system also provides many other services, such as
  facilitating access to hardware devices.

“Accessing specific CPU instructions and writing custom assembly language also
require unsafe. This would enable us to use vector instructions for processing
multiple values at the same time.”

“There are also lots of interesting things that you might want to build that
require unsafe blocks. Do you have any ideas?”

- advanced data structures such as concurrency primitives and cyclic data
  structures
- memory allocators
- programming languages! (Safe Rust is written with Unsafe Rust)

</details>

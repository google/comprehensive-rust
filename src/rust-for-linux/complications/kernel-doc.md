---
minutes: 3
---

# Documentation

Documentation in Rust for Linux is built with the `rustdoc` tool just like for
regular Rust code.

Running rustdoc on the kernel is done with the `rustdoc` Make target:

```sh
$ make LLVM=1 rustdoc
```

after which generated docs can be viewed by opening
`Documentation/output/rust/rustdoc/kernel/index.html`.

Pre-generated documentation for the current kernel release is available at:

<https://rust.docs.kernel.org/kernel/>

## More information

<https://docs.kernel.org/rust/general-information.html#code-documentation>

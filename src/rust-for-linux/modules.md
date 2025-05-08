---
minutes: 5
---

# Building Kernel Modules

To build kernel modules in Rust, we need to build `.ko` shared objects that link
against the rest of the kernel.

In C, kernel modules use the `module_init` and `module_exit` macros to specify
how to initialize and deinitialize the module. For Rust, we'll need some
equivalent of these macros. Ultimately, these macros specify the values of two
fields in a `struct this_module` which is placed in the
`.gnu.linkonce.this_module` section of the kernel module object file.

We can achieve this in Rust with by defining an equivalent struct as a `static`
item and using the `#[unsafe(link_section = ".gnu.linkonce.this_module")]`
attribute. The `.init` and `.exit` fields of our struct will need to be pointers
to the appropriate functions, which leads to our next question: how do we define
Rust functions with the types and calling convention expected here?

In practice, Rust for Linux has a convenient and safe wrapper around this
pattern which we'll see when we look at a real-world Rust kernel module.

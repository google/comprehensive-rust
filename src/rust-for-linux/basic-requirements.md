# Interoperation Requirements

To use Rust code in Linux, we can start by comparing this situation with C/Rust
interop in userspace.

## Building

In userspace, the most common setup is to use Cargo to compile our Rust and
later integrate into a C build system if needed.
Meanwhile, the Linux Kernel compiles its C code with its custom Kbuild build
system.
In Rust for Linux, the kernel build system invokes the Rust compiler directly,
without Cargo.

## No `libstd`

Unlike typical usage of Rust in userspace, which makes use of the rust standard
library through the `std` crate, Rust in the kernel does not run atop an
operating system, so kernel Rust will have to eschew the standard library.

## Module Support

Much code in the kernel is compiled into kernel modules rather than as part of
the core kernel.
To write kernel modules in Rust we'll need to be able to match the ABI of kernel
modules.

## Safe Wrappers

To reap the benefits of Rust, we want to be able to write as much code as
possible in safe Rust.
This means that we want safe wrappers for as much kernel functionality as
possible.

## Mapping Types

When writing these wrappers, we'll need to refer to the data types of values
passed to and from existing kernel functions in C.
Unlike userspace C, the kernel uses its own set of primitive types rather than
those provided by the C standard.
We'll have to map back and forth between those kernel types and compatible Rust
ones when doing foreign calls.

## Keeping the Kernel Lean

Finally, even the core Rust library assumes a basic level of functionality that
includes some costly operations (e.g. unicode processing) for which the kernel
does not want to pay implementation costs.
To use Rust in the kernel we'll need a way to disable this functionality.

# Outline

{{%segment outline}}

# Initialization

> TODO(timclicks): split this content into multiple sub-sections

---

## Addressing data

```rust
static s: &str = "_";

fn main() {
    let l = 123;
    let h = Box::new(123);

    println!("{:p}", &l);
    println!("{:p}", s);
    println!("{:p}", &*h);
}
```

<details>

All data stored in a program lives at an _address_, a number which the operating
system can use to retrieve or store data at that address.

Local variables, such as `l`, are stored on the "stack". Memory addresses on the
stack are quite high. (When executed, the program probably prints out a value
near `0x7fffffffffff`)

Static variables are lower

Functions also stored in memory. In Rust, the keyword `fn` signifies a function
pointer. Its address can also be printed.

### Questions

- Q: Why does addresses printed a not start at 1?\
  A: The kernel reserves half of a process's address space for itself in the
  lower half.

### Variable mapping

- `l` - L for _local_ - stored on the "stack"
- `h` - H for _heap_
- `f` - F for _function_
- `s` - S for _static_

</details>

---

## Memory lifecycle

Unpaged

Mapped but unallocated

Allocated

Allocated and "available" (uninitialized)

Allocated and "active" (ininitialized)

Call to free

Deallocated but mapped

Unpaged

<details>

Variables, the data that is used to represent them, have a surprisingly complex
lifecycle.

Operating systems, programming languages and hardware cooperate to programs with
convenient access to data stored on physical devices, such as RAM chips.
Programs are provided with a façade, an imaginary array of bytes addressed from
1 to _n_, that allows them to store and retrieve data.

This imaginary array of bytes is called the _virtual address space_ and this
setup is called _virtual memory_.

Each operating system process has its own virtual address space, meaning that
the same address means different things in different processes. Another way of
thinking about this is that process believes that it has exclusive access to the
data available to the machine.

The operating system kernel is responsible for mapping between these virtual
memory addresses that your program understands to something that the hardware
understands.

To do this bookkeeping, the kernel stores information in its own data structures
and relies on concept of a _memory page_. Pages are typically 4 KB in size
(although this can be tuned).

Virtual memory is complex and has many stages.

The kernel understands physical memory addresses. User-space programs only have
access to virtual memory.

The details are complex and we don't want to turn this class into a
graduate-&spy;level computer architecture course. However, understanding this
system is useful, because it explains why programmers use uninitialized memory
for performance-critical code.

The mapping between memory addresses and the pages themselves is also stored
within memory, in a data structure that is called TLB. TLB expands to
"thread-local buffer", which is a name that has persisted for historical
reasons.

The CPU provides the operating system with privileged instructions for
interacting with hardware, including main memory.

Rust's ownership model adds its own characteristics to this overall model. The
data is likely to still be present in the original location, after variables are
moved, however this is inaccessible to the program.

## References

An extensive introduction background

Drepper, Ulrich (2007) "What every programmer should know about memory"

The Linux kernel provides extensive documentation about how virtual memory works
on each platform https://www.kernel.org/doc/html/v5.8/x86/x86_64/mm.html

</details>

---

> All runtime-allocated memory in a Rust program begins its life as
> uninitialized.
>
> &mdash;
> [The Rustonomicon](https://doc.rust-lang.org/nomicon/uninitialized.html)

<details>

Validity related to other concepts that we've seen before, such as _undefined
behavior_. Validity is a precondition for well-defined behavior.

This segment of the course describes what initialization is and some of its
related concepts, such as _alignment_ and _validity_, and how they relate to one
that we've seen before: _undefined behavior_.

The primary focus of the segment though is to introduce the
`std::mem::MaybeUninit` type. Its role is to allow programmers to interact with
memory that is uninitialized and convert it to some initialized state.

To get this to work, we'll work through several code examples and other
exercises.

---

```rust,editable
fn mystery() -> u32 {
    let mut x: u32;

    unsafe { x }
}

fn main() {
    let a = mystery();
    println!("{a}")
}
```

<details>

What is the value of `x`?

**Action:** Pause and await for people's responses.

We can't know.

This is a case of an _uninitialized_ value. When we define the variable on line
2, the compiler makes space for an integer on the stack, however it makes no
guarantees that there is a valid value there.

**Action:** Attempt compilation.

**Action:** Suggested change:

```rust
use std::mem;

fn mystery() -> u32 {
    let mut x: u32 = unsafe { mem::MaybeUninit::uninit().assume_init() };

    x
}

fn main() {
    let a = mystery();
    println!("{a}")
}
```

Initialization transforms that a value's bytes from an undetermined state to
something that's guaranteed to be valid.

As we've seen from the Boolean case, not every bit pattern is a valid value in
Rust's `bool` type.

When a value uninitialized, it's impossible to know what'.

Rust requires every variable is _valid_. An important part of validity is
ensuring that values are initialized before use.

Getting this wrong is so unsafe that you cannot simply use the `unsafe` keyword
to convince Rust to compile your code.

</details>

---

## Validity

- What is validity?
- Why is it important?

<details>

This segment of the course describes what that means and why it's important.

Validity related to other concepts that we've seen before, such as _undefined
behavior_. Validity is a precondition for well-defined behavior.

</details>

---

## Validity

<svg width="400" height="300" xmlns="http://www.w3.org/2000/svg">\
<circle cx="200" cy="150" r="160"
          fill="rgba(70, 130, 180, 0.3)"
          stroke="rgba(70, 130, 180, 0.8)"
          stroke-width="2"/>\
<circle cx="200" cy="150" r="120" fill="rgba(255, 165, 0, 0.4)" stroke="rgba(255, 140, 0, 0.8)" stroke-width="2"/>\
<text x="200" y="60" text-anchor="middle" font-size="18" fill="rgba(70, 130, 180, 1)">
Bit patterns </text>\
<text x="200" y="150" text-anchor="middle" font-size="18" fill="rgba(255, 140, 0, 1)">
Valid values </text>\
</svg>

<details>

Data types define what it means to be _valid_. For some types, such as integers,
every bit pattern is a valid type. For many others though, there are some
patterns which are not.

In Rust, references are not allowed to be NULL and `char` values must be valid
Unicode scalar values.

Outside of bit patterns, there are also other considerations. For example, many
types impose rules that must be enforced that extend past. The way to find these
rules is by the documentation. Therefore, we're also going to spend time
examining docs.

</details>

---

## Why `MaybeUninit<T>`?

```rust,editable
```

<details>

Rust requires every variable to be initialized before use. More generally,
compilers assume that all variables are properly initialized.

But for FFI and for creating high performance data structures&mdash;sometimes
referred to as getting stuff done&mdash;we need the ability to describe
uninitialized buffers.

</details>

---

## Why care about initialization?

```rust,editable
fn create_1mb_buffer() -> Vec<u8> {
    vec![0; 1_000_000]
}
```

<details>

You're probably aware that this code allocates a new block of memory. It also
has a second phase that is slightly more subtle. After allocation, every byte
has its bits set to zero.

However, there are cases where this second step is unnecessary. For example, if
we're using this buffer for I/O, then we're going to overwrite the memory with
whatever data that is going to be provided.

</details>

---

## Case study: selective initialization

```rust
use std::mem::MaybeUninit;

/// Builds a sparse row where only certain positions have values
struct ArrayFastBuilder<const N: usize> {
    data: [MaybeUninit<f64>; N],
    initialized: [bool; N],
    count: usize,
}

impl<const N: usize> ArrayFastBuilder<N> {
    fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            initialized: [false; N],
            count: 0,
        }
    }

    fn set(&mut self, index: usize, value: f64) -> Result<(), &'static str> {
        if index >= N {
            return Err("Index out of bounds");
        }

        if !self.initialized[index] {
            self.count += 1;
        }

        self.data[index] = MaybeUninit::new(value);
        self.initialized[index] = true;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<f64> {
        if index < N && self.initialized[index] {
            Some(unsafe { self.data[index].assume_init() })
        } else {
            None
        }
    }

    fn into_array(self, default: f64) -> [f64; N] {
        let mut result: [MaybeUninit<f64>; N] = std::array::from_fn(|i| {
            if self.initialized[i] {
                self.data[i] // Already initialized
            } else {
                MaybeUninit::new(default)
            }
        });

        unsafe {
            std::ptr::read(
                &result as *const [MaybeUninit<f64>; N] as *const [f64; N],
            )
        }
    }

    fn into_sparse_vec(self) -> Vec<(usize, f64)> {
        let mut result = Vec::with_capacity(self.count);

        for (i, is_init) in self.initialized.iter().enumerate() {
            if *is_init {
                let value = unsafe { self.data[i].assume_init() };
                result.push((i, value));
            }
        }

        result
    }
}
```

<details>

Here is an application of what we just saw. `ArrayFastBuilder` reserves space on
the stack for the contents, but skips avoids zeroing that array when it is
created.

</details>

---

## What is the contract?

Whenever we're creating unsafe code, we need to consider what the contract is.

What does `assume_init(self)` mean? What do we need to do to guarantee that
initialization it is no longer an assumption.

<details>

What is this code asking of us? What are the expectations that we need to
satisfy? If we don't know the expectations, where would we find them?

</details>

---

## Layout guarantees

The following program runs successfully for `u64` values. Is that the case for
all possible types `T`?

```rust,editable
use std::mem::MaybeUninit;

fn main() {
    
    let u = MaybeUninit<u64>::uninit();

    assert_eq!(size_of::<MaybeUninit<u64>>(), size_of::<u64>());
    assert_eq!(align_of::<MaybeUninit<u64>>(), align_of::<u64>());    
}
```

Look through the documentation for `MaybeUninit` to verify your assumptions.

<details>

Another way to ask this is to check whether guarantees does `MaybeUninit<T>`
provide about its memory layout?

Here is [the relevant quote][q] from the Layout section of the docs:

> `MaybeUninit<T>` is guaranteed to have the same size, alignment, and ABI as
> `T`.

[q]: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#layout-1

</details>

---

## What about safety when panicking?

```rust
```

<details>

Rust's drop behavior presents a challenge during panics. In situations where
there is partially-initiated values, dropping causes undefined behavior.

</details>

---

## Questions for review

Where should the safety comment be? What kinds of tests can we perform. Fuzzing.

---

## Exercise: Vec<T>

Look up the documentation for `assume_init` and describe why this creates
undefined behavior:

```rust
use std::mem::MaybeUninit;

fn main() {
    let x = MaybeUninit::<Vec<u32>>::uninit();
    let x_ = unsafe { x.assume_init() };

    println!("{x_:?}")
}
```

<details>

Many types have additional invariants that need to be upheld. For example,
`Vec<T>` has a different representation when it's first created with `::new()`
compared to after its first entry is inserted. It lazily allocates memory and
there is no allocation involved until space is actually needed.

From the [doc comment of `assume_init()`][docs]:

> It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an
> initialized state. Calling this when the content is not yet fully initialized
> causes immediate undefined behavior. The type-level documentation contains
> more information about this initialization invariant.
>
> On top of that, **remember that most types have additional invariants beyond
> merely being considered initialized at the type level**. For example, a
> 1-initialized `Vec<T>` is considered initialized (under the current
> implementation; this does not constitute a stable guarantee) because the only
> requirement the compiler knows about it is that the data pointer must be
> non-null. Creating such a `Vec<T>` does not cause immediate undefined
> behavior, but will cause undefined behavior with most safe operations
> (including dropping it).
>
> _Emphasis added_

[docs]: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.assume_init

### Extension exercise

Ask the class to think of other types that require special handling:

- `char` outside the range of a Unicode scalar
  (`[0x0000..=0xD7FF, 0xE000..=0x10FFFF]`)
- References, (NULL is a valid pointer, but not a valid reference)
- Types backed by `Vec<_>`, including `String`.
- Pinned types, i.e. `Pin<T>`
- Non-zero types, i.e. `NonZeroU32`, etc

</details>

---

## MaybeUninit use case: initializing a struct field by field

```rust
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

#[derive(Debug, PartialEq)]
pub struct FileFormat {
    marker: [u8; 4],
    len: u32,
    data: Vec<u8>,
}

fn main() {
    let rfc = {
        let mut uninit: MaybeUninit<Foo> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        unsafe {
            addr_of_mut!((*ptr).name).write([b'R', b'F', b'C', b'1']);
        }

        unsafe {
            addr_of_mut!((*ptr).len).write(3);
        }

        unsafe {
            addr_of_mut!((*ptr).list).write(vec![0, 1, 2]);
        }

        unsafe { uninit.assume_init() }
    };

    assert_eq!(
        rfc,
        FileFormat {
            name: b"RFC1",
            len: 3
            data: vec![0, 1, 2]
        }
    );
}
```

---

## Use case: partial initialization

```rust,editable
use std::mem::MaybeUninit;

const SIZE: usize = 10_000_000;

fn with_zeroing() -> Vec<u8> {
    let mut vec = vec![0u8; SIZE];
    for i in 0..SIZE {
        vec[i] = (i % 256) as u8;
    }
    vec
}

fn without_zeroing() -> Vec<u8> {
    let mut vec = Vec::with_capacity(SIZE);
    unsafe {
        let ptr = vec.as_mut_ptr();
        for i in 0..SIZE {
            ptr.add(i).write((i % 256) as u8);
        }
        vec.set_len(SIZE);
    }
    vec
}
```

<details>

</details>

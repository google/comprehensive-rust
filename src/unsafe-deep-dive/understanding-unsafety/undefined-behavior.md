---
minutes: 30
---

# Example: Representing Boolean values

> TODO(timclicks): split this content into multiple sub-sections

One of the terms that we introduced earlier was _undefined behavior_. This
exercise aims to discuss what undefined behavior actually is and how it can
arise.

High performance code is particularly prone to accidentally introducing
undefined behavior into a program, because its authors are typically very
interested in finding ways to cut corners.

---

## What's wrong with undefined behavior?

C++ compilers will typically (*) compile this code without warnings, and will
run without error signaling an error:

```cpp
#include <cassert>

int axiom_increment_is_greater(int x) {
    return x + 1 > x;
}

int main() {
    int a = 2147483647;
    assert(axiom_increment_is_greater(a));
}
```

Equivalent Rust programs produce different output:

```rust,editable
fn axiom_increment_is_greater(x: i32) -> bool {
    x + 1 > x
}

fn main() {
    let a = 2147483647;
    assert!(axiom_increment_is_greater(a));
}
```

(*) We can't be certain. That's one of the problems.

<details>

We don't want to have undefined behavior in our code, because it makes the code
_unsound_.

Unsound code can crash abruptly or produce unexpected results, because compilers
are written with the assumption that undefined behavior does not exist. They
will create optimizations that could be completely contrary to your
expectations.

In this example, assume that we're creating some sort of proof assistant that
makes deductions based on mathematical axioms. One of the axioms that we want to
encode is that an integer's increment is always greater than the integer itself:

gcc v13.2, clang v16.0.0 and msvc v19.0 [all compile the C++ code to][asm] the
following assembly when optimizations are enabled ( `-O2`):

```asm
axiom_increment_is_greater(int):
        mov     eax, 1
        ret
```

[asm]: https://godbolt.org/z/q4MMY8vxs

That is, while it looks like they'll always return `true`, the code also
produces undefined behavior. When `x` is 2^32-1 and is incremented, it enters an
undefined state. The operation produces a number that is outside of the range of
a 32-bit signed integer.

Integer overflow for signed integers is _undefined_. In the conventional twos
complement representation, increment often wraps to -(2^31)-1 `i32::MIN`.

Rust takes a stricter approach. When integer oveflow is signaled by the CPU, a
panic is induced. This allows Safe Rust to be free of undefined behaviour.

</details>

---

## Rust keeps undefined behavior out...

...but, unsafe provides a way for it to get back in.

<details>

We are going to work through an example of how undefined behavior can be
introduced in an attempt to improve performance.

</details>

---

## Booleans

A typical representation:

- 1 => truth/positivity
- 0 => falsehood/negativity

<details>

## Discussion

### Encoding

Just as integers can have their quirks, so do Boolean data types.

How are the Boolean values `true` and `false` represented by programming
languages?

Many languages, including Rust and C++, encode Boolean values as an integer,
where:

- 1 represents truth or positivity
- 0 represents falsehood or negativity

However, there is an impedance mismatch because even the smallest integer (a
single byte) can represent many more numbers than the two that are required.

> Aside: Not a universal definition
>
> Programming language designers are free to have their own representations, or
> not include a Boolean type in their language at all.
>
> CPUs do not have a Boolean datatype, rather they have Boolean operations that
> are performed against operands that are typically integers.

As the input space is larger than the output space, this can cause problems.
Allowing any byte to represent "true", except for `0x01`, is undefined.

[Rust (following C) imposes the following restrictions][ref-bool] on its `bool`
type:

> The value `false` has the bit pattern `0x00` and the value `true` has the bit
> pattern `0x01`. It is _undefined behavior_ for an object with the boolean type
> to have any other bit pattern. [emphasis added]

[ref-bool]: https://doc.rust-lang.org/reference/types/boolean.html

Depending on one's perspective, this either presents an opportunity or a
difficulty.

</details>

---

## Exercise

- Define a type that represents a Boolean value
- A zero-cost conversion function from `u8` to your new type
- A zero-cost conversion function from your new type to `bool`

```rust,editable
fn byte_to_boolean(byte: u8) -> Boolean {
    todo!("convert from u8")
}

fn boolean_to_bool(b: Boolean) -> bool {
    todo!("convert to Rust's bool")
}
```

<details>

Tell the group that they will need start by defining the `Boolean` type that's
provided in the type signature themselves. (This is not included in the sample
code so that the audience is not biased using a `struct`)

This exercise should be completed quite quickly &ndash; no more than 3 minutes
&ndash; because we will soon review several examples ourselves.

### Recommended guidance

- User-defined Booleans should occupy a single byte the same space. This
  precludes using an `enum`.
- The following function annotations are likely to be needed:
  - `unsafe` on the `byte_to_boolean` function
  - `#[inline]`
- Rust's `Copy` trait involves memcpy and is therefore _not_ zero-cost

> _Aside: Possible upcoming language feature - the TransmuteFrom trait_
>
> The standard library contains a nightly feature, [`transmutability`] which
> defines the [`std::mem::TransmuteFrom`] trait for performing this kind of
> operation. This is one of the outputs from the [Safe Transmute Project] within
> the Rust compiler team.

[`std::mem::TransmuteFrom`]: https://doc.rust-lang.org/std/mem/trait.TransmuteFrom.html
[`transmutability`]: https://github.com/rust-lang/rust/issues/99571
[Safe Transmute Project]: https://github.com/rust-lang/project-safe-transmute

### Questions to raise

- How would we indicate to callers that they can cause undefined behavior by
  calling `byte_to_boolean` with invalid inputs?
  - Safety comments. You could briefly mention safety comments and raise
    questions about what learners would expect to see if they were reviewing
    code.
  - Adding assertions. While not a complete solution, you can suggest that
    learners add assertions under debug and/or test.

### Partial solution focusing on assertions

```rust
fn is_valid_bool_repr(byte: u8) -> bool {
    (byte >> 1) != 0
}

fn byte_to_boolean(byte: u8) -> Boolean {
    if cfg!(debug_assertions) || cfg!(test) {
        assert!(is_valid_bool_repr(byte), "input must be 0x00 or 0x01")
    }

    todo!("convert from u8")
}
```

### Full solution

```rust
struct Boolean(bool);

fn is_valid_bool_repr(byte: u8) -> bool {
    (byte >> 1) != 0
}

/// Create a `Boolean` from a `u8`
///
/// ## Safety
///
/// This function produces undefined bahavior when `byte` is neither 0 nor 1.
unsafe fn byte_to_boolean(byte: u8) -> Boolean {
    if cfg!(debug_assertions) || cfg!(test) {
        assert!(is_valid_bool_repr(byte), "input must be 0x00 or 0x01")
    }

    // SAFETY: Valid for all valid inputs into this function
    let b = unsafe { std::mem::transmute(byte) };
    Boolean(b)
}

fn boolean_to_byte(b: Boolean) -> bool {
    b.0
}

fn main() {
    let t = 123;
    let ub = unsafe { byte_to_boolean(t) };
    if boolean_to_byte(ub) {
        println!(r"¯\_(ツ)_/¯");
    }
}
```

### Picking a data structure

**Newtype wrapping u8**

The orthodox strategy will be to wrap `u8` in a struct:

```rust
struct Boolean(u8);
```

This ensures that the representation is the same as `u8`.

**Newtype wrapping bool**

Hopefully, you will have some learners will wrap `bool` as a newtype:

```rust
struct Boolean(bool);
```

At first, this may look like a bit of a cheat code for the exercise. It won't
avoid the need to convert from `u8`, however.

Wrapping `bool` includes the bonus that you can guarantee--in so far as you can
guarantee Rust's own behavior--that `Boolean` is spec-compliant with `bool`.

It may also look redundant - why bother creating a new type when it doesn't
perform as a `bool`? Because it gives us complete control over the trait system.

**Union**

An alternative strategy would be to use a `union`:

```rust
union Byte {
    u8,
    bool,
}
```

This isn't advised. It means that the value will _never_ be able to be
considered safe to access. Callers will need to ensure that they comply with the
rules at every interaction with the type.

**Typestate**

Some advanced programmers may attempt to encode Boolean values as zero-sized
types in the type system. If you receive questions about this, gently nudge them
back to including the byte.

```rust
struct True;
struct False;
```

There are a couple of reasons for this. First, zero-sized types do not obey the
width and alignment requirements of the spec for `bool`. Secondly, they're very
difficult to work with in practice.

If they wish to make use of the typestate pattern, then a possible alternative
would be to create three independent types. This creates an ergonomic problem,
but might that might be justified if you only want to permit a follow-on
function from being only called from a "true" value.

```rust
struct True(bool);
struct False(bool);
```

</details>

---

## Code reviews

We'll now be critiquing other implementations of the previous exercise.

<details>

The critical point in these reviews is that learners accurately describe the
contract that callers need to uphold when converting from `u8`. It should be
well described in a Safety section of the docstring.

</details>

---

## Code review 1

Critique this code and suggest improvements, if any:

```rust,editable
struct Boolean(u8);

fn byte_to_boolean(b: u8) -> Boolean {
    Boolean(b)
}

fn boolean_to_byte(boolean: Boolean) -> u8 {
    boolean.0
}

fn boolean_to_bool(boolean: Boolean) -> bool {
    match b.0 {
        0 => false,
        _ => true,
    }
}
```

<details>

Which function should be `unsafe`? It could either be at the "constructor"
(`byte_to_boolean`) or when the Boolean is converted to a Rust-native `bool`
(`boolean_to_bool`).

</details>

---

## Code review 2

```rust,editable
struct Boolean(bool);

fn byte_to_boolean(b: u8) -> Boolean {
    match b.0 {
        0 => Boolean(false),
        _ => Boolean(true),
    }
}

fn boolean_to_byte(boolean: Boolean) -> u8 {
    boolean.0 as u8
}

fn boolean_to_bool(boolean: Boolean) -> bool {
    boolean.0
}
```

<details>

In this version, we mask the error. All non-zero inputs are coerced to `true`.
We store the internal field of the `Boolean` struct as a `bool` to make as much
use of Rust's type system as possible.

However, this `byte_to_boolean` is not zero-cost. There is still a `match`
operation that's required.

</details>

---

## Code review 3

```rust,editable
#[repr(C)]
union Boolean {
    raw: u8,
    rust: bool,
}

fn byte_to_boolean(b: u8) -> Boolean {
    Boolean { raw: b }
}

fn boolean_to_byte(boolean: Boolean) -> u8 {
    unsafe { boolean.rust }
}

fn boolean_to_bool(boolean: Boolean) -> bool {
    unsafe { boolean.raw }
}
```

---

## Code review 4

```rust,editable
struct Boolean(bool);

fn byte_to_boolean(b: u8) -> Boolean {
    let b: bool = unsafe { sys::mem::transmute(b) };

    Boolean(b)
}

fn boolean_to_byte(boolean: Boolean) -> u8 {
    boolean.0 as u8
}

fn boolean_to_bool(boolean: Boolean) -> bool {
    boolean.0
}
```

---

## Scratch Space

> Note: Content following this comment is from a previous revisions and is being
> retained temporarily.

> TODO(timclicks): Review the following content for anything useful that should
> be retained.

---

Or in Rust syntax:

```rust
struct Boolean(u8);

const true: Boolean = Boolean(1);
const false: Boolean = Boolean(0);
```

>> Instructor Notes
>
> We define a type here so that there is no confusion in the type system between
> `u8` and `Boolean`.

From a theoretical perspective, the two states `true` and `false` be represented
by a single bit. However, the smallest integer available is `u8`, which has 254
additional states.

This is a similar problem to the mismatch casting from a `i64` to `i32`, but
there is a significant difference. When converting an integer from a 64-bit type
to a 32-bit type, there is not enough space in the narrower type for all
possible input values. They can't all fit. In the case of casting from `u8` to
`bool`, the number of bits isn't the issue. It's the standard that imposes the
additional restrictions.

Depending on one's perspective, this either presents an opportunity or a
challenge.

Moreover, [Rust (following C) imposes the following restrictions][ref-bool] on
its `bool` type:

> The value `false` has the bit pattern `0x00` and the value `true` has the bit
> pattern `0x01`. It is _undefined behavior_ for an object with the boolean type
> to have any other bit pattern. [emphasis added]

Many CPUs, don't strictly have a "Boolean type". They have Boolean operations.

- For true, CPUs ask. Does this value match

[ref-bool]: https://doc.rust-lang.org/reference/types/boolean.html

## Exercise

Implement two conversion functions, `byte_to_boolean()` and `boolean_to_byte()`:

```rust
struct Boolean(u8);

fn byte_to_boolean(b: u8) -> Boolean {
    todo!();
}

fn boolean_to_byte(b: Boolean) -> u8 {
    todo!();
}
```

## Discussion

Should this function be marked as unsafe?

```rust
struct Boolean(u8);

fn byte_to_boolean(b: u8) -> Boolean {
    match b {
        0 => false,
        _ => true,
    }
}
```

---

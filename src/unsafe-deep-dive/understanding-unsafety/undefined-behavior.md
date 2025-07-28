---
minutes: 15
---

# Example: Representing Boolean values

> TODO(timclicks): split this content into multiple sub-sections

One of the terms that we introduced earlier was _undefined behavior_. This
exercise aims to discuss what undefined behavior actually is and how it can
arise.

High performance code is particularly prone to introducing undefined behavior
into a program, because it will typically find every corner that's possible to
cut.

We don't want to have undefined behavior in our code, because it makes the code
_unsound_. Unsound code can crash abruptly or produce unexpected results,
because compilers are written with the assumption that undefined behavior does
not exist.

Safe Rust does not permit undefined behavior.

It becomes impossible to reason about, .

In fact, compilers are engineered to assume that undefined behavior never
exists.

We are going to work through an example of how undefined behavior can be
introduced in an attempt to improve performance.

---

## Part 1

How are the Boolean values `true` and `false` represented by programming
languages?

Many languages, including Rust and C, encode Boolean values as an integer,
where:

- 1 represents truth or positivity
- 0 represents falsehood or negativity

### Exercise:

Define a type that represents a bool

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

is a mismatch when casting between a `u8` and a `bool`.

That means, to covert from an integer to bool.

Depending on one's perspective, this either presents an opportunity or a
challenge.

Moreover, [Rust (following C) imposes the following restrictions][ref-bool] on
its `bool` type:

> The value `false` has the bit pattern `0x00` and the value `true` has the bit
> pattern `0x01`. It is _undefined behavior_ for an object with the boolean type
> to have any other bit pattern. [emphasis added]

Many CPUs, don't strictly have a "Boolean type". They have Boolean operations.

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

## 

This example demonstrates how the search for high performance can . Software
engineers can find themselves wanting to exploit characteristics of the
operating environment,

CPUs

> Well, actually...
>
> CPUs don't really have a concept of a Boolean value. Instead, they have
> Boolean operations.

In Rust, the conventional way to think of them is something like this:

They're encoded as

Boolean values must match a precise representation to avoid undefined behavior:

<table>
    <tr>
        <th>Bit pattern</th><th>Rust type</th>
    </tr>
    <tr>
        <td><code class="hljs">00000001</code></td><td><code class="hljs">true</code></td>
    </tr>
    <tr>
        <td><code class="hljs">00000000</code></td><td><code class="hljs">false</code></td>
    </tr>
    <tr>
        <td>Other patterns</td><td>Undefined</td>
    </tr>
</table>

You have two tasks in this exercise.

- First,
  - Create Rust type, `Boolean` type that represents a Boolean value in a
    spec-compliant way
  - The first create values of your type from `u8` with no overhead cost while
    ensuring that undefined behavior is impossible.
- Secondly, review someone else's implementation.

Starter code:

Part 1 involves a `Boolean`, which is a type that can be

<details>

Admittedly, there isn't much starter code.

## Discussion

- The critical point in these reviews is that learners accurately describe the
  contract that callers need to uphold when converting from `u8`. It should be
  well described in a Safety section of the docstring.
- Functions should have an `#[inline(always)]` annotation as Rust's `Copy` trait
  involves memcpy. We want the compiler to erase the function call

> _Aside: TransmuteFrom trait_
>
> The standard library contains a nightly feature, `transmutability` which
> defines the [`std::mem::TransmuteFrom`] trait for performing this kind of
> operation. This is one of the outputs from the [Safe Transmute Project] within
> the Rust compiler team.

[`transmutability`]: https://github.com/rust-lang/rust/issues/99571
[Safe Transmute Project]: https://github.com/rust-lang/project-safe-transmute
[`std::mem::TransmuteFrom`]: https://doc.rust-lang.org/std/mem/trait.TransmuteFrom.html

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
would be to .

```rust
struct Boolean(bool);
struct True(bool);
struct False(bool);
```

## Code review

Suggest that there be some advice

</details>

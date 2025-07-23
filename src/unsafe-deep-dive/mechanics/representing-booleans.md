# Example: Representing Boolean values

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

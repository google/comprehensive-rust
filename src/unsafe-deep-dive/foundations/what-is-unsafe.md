# What is &ldquo;unsafety&rdquo;?

Rust is intentionally vague about what "unsafe" means.

Let's create a working definition together.

<details>

## Clarifying "intentionally vague"

It's likely that you'll have an audience member object to the phrase
"intentionally vague". Perhaps refer to this statement from the standard library
when discussing the [safety requirements of raw pointers].

> Many functions in [this module] take raw pointers as arguments and read from
> or write to them. For this to be safe, these pointers must be _valid_ for the
> given access.
>
> ...
>
> The precise rules for validity are not determined yet.

[this module]: https://doc.rust-lang.org/std/ptr/index.html
[safety requirements of raw pointers]: https://doc.rust-lang.org/std/ptr/index.html#safety

## Definitions from authoritative docs:

stdlib's keyword:

> Code or interfaces whose memory safety cannot be verified by the type system.
>
> ...
>
> Here are the abilities Unsafe Rust has in addition to Safe Rust:
>
> - Dereference raw pointers
> - Implement unsafe traits
> - Call unsafe functions
> - Mutate statics (including external ones)
> - Access fields of unions

From the [reference](https://doc.rust-lang.org/reference/unsafety.html)

> The following language level features cannot be used in the safe subset of
> Rust:
>
> - Dereferencing a raw pointer.
> - Reading or writing a mutable or external static variable.
> - Accessing a field of a union, other than to assign to it.
> - Calling an unsafe function (including an intrinsic or foreign function).
> - Calling a safe function marked with a target_feature from a function that
>   does not have a target_feature attribute enabling the same features (see
>   attributes.codegen.target_feature.safety-restrictions).
> - Implementing an unsafe trait.
> - Declaring an extern block.
> - Applying an unsafe attribute to an item.

## Group exercise

> You may have a group of learners who are not familiar with each other yet.
> This is a way for you to gather some data about their confidence levels and
> the psychological safety that they're feeling.

### Part 1: Informal definition

> Use this to gauge the confidence level of the group. If they are uncertain,
> then tailor the next section to be more directed.

Ask the class: **By raising your hand, indicate if you would feel comfortable
defining unsafe?**

If anyone's feeling confident, allow them to try to explain.

### Part 2: Evidence gathering

Ask the class to spend 3-5 minutes.

- Find a use of the unsafe keyword. What contract/invariant/pre-condition is
  being established or satisfied?
- Write down terms that need to be defined (unsafe, memory safety, soundness,
  undefined behavior)

### Part 3: Write a working definition

### Part 4: Remarks

Mention that we'll be reviewing our definition at the end of the day.

</details>

# Safety checklist

When writing and reviewing `unsafe` code, we should make sure that we've
considered the following considerations _and documented_ what callers must later
uphold:

- Validity
- Alignment
- Lifetimes
- Ownership
- Platform
- Compliance with specifications

<details>

**Validity**

Callers must ensure that values must match some bit-pattern.

**Alignment**

Callers must ensure that values are correctly aligned.

**Lifetimes**

Do callers need to verify that a referent must exist before/after/during?

**Ownership**

Can this function generate confusion about ownership?

> _Aside:_ Memory leaks
>
> A discussion about leaking memory may arise here. If calling a function
> removes all ownership information, then .
>
> Memory leaking is not strictly a memory safety concern. However, it's often a
> problem in practice, especially if it is unintentional.
>
> Therefore, this should at least be documented. If it's possible to mishandle
> the API and cause an unintentional leak, then there is a case for an unsafe
> block.

**Platform**

Callers must be wary of platform-specific behavior.

**Compliance with specifications**

"Business Rules", i.e. all values must be even numbers.

</details>

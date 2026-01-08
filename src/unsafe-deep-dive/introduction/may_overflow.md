---
minutes: 10
---

# Example: may_overflow function

```rust,should_panic,editable
/// Adds 2^31 - 1 to negative numbers.
unsafe fn may_overflow(a: i32) -> i32 {
    a + i32::MAX
}

fn main() {
    let x = unsafe { may_overflow(123) };
    println!("{x}");
}
```

<details>

“The `unsafe` keyword may have a subtly different meaning than what some people
assume.”

“The code author believes that the code is correct. In principle, the code is
safe.”

“In this toy example, the `may_overflow` function is only intended to be called
with negative numbers.

Ask learners if they can explain why `may_overflow` requires the unsafe keyword.

“In case you’re unsure what the problem is, let’s pause briefly to explain. An
`i32` only has 31 bits available for positive numbers. When an operation
produces a result that requires more than 31 bits, then the program is put into
an invalid state. And it’s not just a numerical problem. Compilers optimize code
on the basis that invalid states are impossible. This causes code paths to be
deleted, producing erratic runtime behavior while also introducing security
vulnerabilities.

Compile and run the code, producing a panic. Then run the example in the
playground to run under `--release` mode to trigger UB.

“This code can be used correctly, however, improper usage is highly dangerous.”

“And it's impossible for the compiler to verify that the usage is correct.”

This is what we mean when we say that the `unsafe` keyword marks the location
where responsibility for memory safety shifts from the compiler to the
programmer.

</details>

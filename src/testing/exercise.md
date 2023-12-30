---
minutes: 30
---

# Exercise: Luhn Algorithm

# Luhn Algorithm

The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is used to
validate credit card numbers. The algorithm takes a string as input and does the
following to validate the credit card number:

- Ignore all spaces. Reject number with less than two digits.

- Moving from **right to left**, double every second digit: for the number
  `1234`, we double `3` and `1`. For the number `98765`, we double `6` and `8`.

- After doubling a digit, sum the digits if the result is greater than 9. So
  doubling `7` becomes `14` which becomes `1 + 4 = 5`.

- Sum all the undoubled and doubled digits.

- The credit card number is valid if the sum ends with `0`.

The provided code provides a buggy implementation of the luhn algorithm, along
with two basic unit tests that confirm that most the algorithm is implemented
correctly.

Copy the code below to <https://play.rust-lang.org/> and write additional tests
to uncover bugs in the provided implementation, fixing any bugs you find.

```rust
{{#include exercise.rs:luhn}}

{{#include exercise.rs:unit-tests}}
}
```

# Luhn Algorithm

The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is used to
validate credit card numbers. The algorithm takes a string as input and does the
following to validate the credit card number:

* Ignore all spaces. Reject number with less than two digits.

* Moving from right to left, double every second digit: for the number `1234`,
  we double `3` and `1`.

* After doubling a digit, sum the digits. So doubling `7` becomes `14` which
  becomes `5`.

* Sum all the undoubled and doubled digits.

* The credit card number is valid if the sum ends with `0`.

Copy the following code to <https://play.rust-lang.org/> and implement the
function:


```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include luhn.rs:luhn}}
    unimplemented!()
}

{{#include luhn.rs:unit-tests}}

#[allow(dead_code)]
fn main() {}
```

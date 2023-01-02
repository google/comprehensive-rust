# Luhn Algorithm
[룬 알고리즘](https://ko.wikipedia.org/wiki/%EB%A3%AC_%EC%95%8C%EA%B3%A0%EB%A6%AC%EC%A6%98)은 신용카드 번호 검증에 사용되는 알고리즘 입니다. 
이 알고리즘은 신용카드 번호를 `문자열`로 입력받고, 아래의 순서에 따라 신용카드 번호의 유효성을 확인합니다:
> The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is used to
> validate credit card numbers. The algorithm takes a string as input and does the
> following to validate the credit card number:

* 모든 공백을 무시합니다, 2자리 미만 숫자는 무시합니다.
* 오른쪽에서 왼쪽으로 이동하며 2번째 자리마다 숫자를 2배 증가시킵니다. `1234`에서 3과 1을 두배로 만듭니다.(2464)
* 두배로 만든 숫자가 2자리가 넘으면 각 자리를 더합니다: `7`의 두배는 `14`이므로 `5`가 됩니다.
* 모든 자리의 숫자를 합합니다.
* 합계의 끝자리가 `0`인 경우 유효한 신용카드 번호 입니다.
> * Ignore all spaces. Reject number with less than two digits.
> * Moving from right to left, double every second digit: for the number `1234`,
>   we double `3` and `1`.
> * After doubling a digit, sum the digits. So doubling `7` becomes `14` which
>   becomes `5`.
> * Sum all the undoubled and doubled digits.
> * The credit card number is valid if the sum is ends with `0`.

아래 코드를 <https://play.rust-lang.org/>에 복사하고 함수를 구현해 보시기 바랍니다.
> Copy the following code to <https://play.rust-lang.org/> and implement the
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

---
역주

- 관련 메서드 확인을 위한 공식문서 서칭만 좀 하면 단순한 스트링 파싱 알고리즘이라...
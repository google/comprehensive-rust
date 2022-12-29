# Scalar Types

|                                          | 타입 Types                                 | 예시 Literals                 |
|------------------------------------------|--------------------------------------------|-------------------------------|
| 부호있는 정수 <br/>Signed integers       | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123i64` |
| 부호없는는 정수 <br/>Unsigned integers   | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10u16`           |
| 부동소수 <br/>Floating point numbers     | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2f32`    |
| 문자열 <br/>Strings                      | `&str`                                     | `"foo"`, `r#"\\"#`            |
| 유니코드 문자 <br/>Unicode scalar values | `char`                                     | `'a'`, `'α'`, `'∞'`           |
| 바이트 문자 <br/>Byte strings            | `&[u8]`                                    | `b"abc"`, `br#" " "#`         |
| 불리언 <br/>Booleans                     | `bool`                                     | `true`, `false`               |

각 타입의 크기는 다음과 같습니다:

* 정수 및 소수형은 뒤의 숫자와 같은 bits 입니다.(`i8`=8 bit)
* `isize` 와 `usize` 는 포인터와 같은 크기입니다.[^역주1]
* 문자는 32 bit 입니다.
* `bool`은 8 bit 입니다. 
> The types have widths as follows:
> 
> * `iN`, `uN`, and `fN` are _N_ bits wide,
> * `isize` and `usize` are the width of a pointer,
> * `char` is 32 bit wide,
> * `bool` is 8 bit wide.

---
역주

[^역주1]: 32비트 시스템에서는 32비트, 64비트 시스템에서는 64비트. C의 int와 같음. 
# Destructuring Enums

구조체의 바인딩된 값을 패턴으로 사용할 수 있습니다. 간단한 `enum` 타입의 예시입니다:
> Patterns can also be used to bind variables to parts of your values. This is how
> you inspect the structure of your types. Let us start with a simple `enum` type:

```rust,editable
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {} into two equal parts", n))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
```

위의 `match`구문을 `divide_in_two`함수에서 반환되는 `Result` 값을 arm[^역주1]으로 분해합니다. 
- 첫번째 arm의 `half`는 `Ok` variant에 바인딩된 값입니다.`(n/2)`
- 두번째 amr의 `msg`는 `Err` variant에 바인딩 된 에러 메시지 입니다.
> Here we have used the arms to _destructure_ the `Result` value. In the first
> arm, `half` is bound to the value inside the `Ok` variant. In the second arm,
> `msg` is bound to the error message.

---
역주
[^역주1]: match의 패턴 중 하나입니다. (switch의 case에 해당)
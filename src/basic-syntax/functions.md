# Functions

러스트 버전의 [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) 함수입니다:
> A Rust version of the famous [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) interview question:

```rust,editable
fn main() {
    // 하단정의. 굳이 상단에 별도 선언할 필요 없음.
    // Defined below, no forward declaration needed
    fizzbuzz_to(20);   
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        // 코너 케이스.
        // Corner case, early return
        return false; 
    }
    // 마지막 표현식은 반환값을 나타냅니다.
    // The last expression is the return value
    lhs % rhs == 0     
}

// `()` 반환값은 반환할 값이 없음을 나타냅니다.
// No return value means returning the unit type `()`
fn fizzbuzz(n: u32) -> () {  
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true,  true)  => println!("fizzbuzz"),
        (true,  false) => println!("fizz"),
        (false, true)  => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

//반환값이 없는 경우(`-> ()`) 생략 가능합니다.
// `-> ()` is normally omitted
fn fizzbuzz_to(n: u32) {  
    for n in 1..=n {
        fizzbuzz(n);
    }
}
```

---
역주
- corner case: 복합 경계 조건. 변수와 환경적인 요소로 인해서 로직에 문제가 있는 경우. ex. 나누기 로직에서 0으로 나누는 경우.
    - edge case: 경계조건. 매개변수 값이 극단적인 최대/최소값(로직 유효범위 끝) 이상인 경우.
- fizzbuzz: 숫자를 입력받아서 n으로 나뉘면 fizz, m으로 나뉘면 buss, 둘다 나뉘면 fizzbuzz, 안나뉘면 입력값을 출력하는 테스트로 자주 쓰는 문제입니다. 예제의 n,m은 3,5
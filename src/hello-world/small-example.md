# Small Example

러스트로 작성된 작은 예제입니다. 
Here is a small example program in Rust:

```rust,editable
fn main() {              // 프로그램 진입점(Program entry point)
    let mut x: i32 = 6;  // 가변 변수 할당(Mutable variable binding)
    print!("{x}");       // printf와 같은 출력 매크로(Macro for printing, like printf)
    while x != 1 {       // 표현식에는 괄호 없음(No parenthesis around expression)
        if x % 2 == 0 {  // 수학식 기호는 다른언어와 유사(Math like in other languages)
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
```


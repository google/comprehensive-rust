# Blocks

러스트에서 블록은 값과 타입을 갖습니다: 블록의 표현식이 값이 됩니다.
A block in Rust has a value and a type: the value is the last expression of the
block:

```rust,editable
fn main() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4
            };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}
```

함수에도 동일한 규칙이 적용됩니다: 함수바디의 (마지막) 값이 반환 값이 됩니다.
The same rule is used for functions: the value of the function body is the
return value:

```rust,editable
fn double(x: i32) -> i32 {
    x + x
}

fn main() {
    println!("doubled: {}", double(7));
}
```
---
역주
- 마지막 줄에 ; 없는 부분을 구문(statements)이 아니라 표현식(expressions)이라고 합니다.
- 블록에서 구문+ 마지막 표현식 인경우 해당 함수를 표현식으로 봅니다. 
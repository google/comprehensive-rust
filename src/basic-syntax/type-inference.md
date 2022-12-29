# Type Inference

러스트는 변수가 어떻게 사용되는지 확인하여 타입 추론을 제공합니다. 
> Rust will look at how the variable is _used_ to determine the type:

```rust,editable
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}
```

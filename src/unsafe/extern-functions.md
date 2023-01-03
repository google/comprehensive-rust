# Calling External Code

다른 언어의 함수는 러스트의 보증을 위반할 수 있습니다. 따라서 이를 호출하는 것은 '안전하지 않습니다':
> Functions from other languages might violate the guarantees of Rust. Calling
> them is thus unsafe:

```rust,editable
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // Abs가 오동작하는 경우 정의되지 않은 동작이 됩니다.
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

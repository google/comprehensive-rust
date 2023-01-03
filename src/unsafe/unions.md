# Unions

유니온타입은 열거형(enum)과 같지만 직접 활성 필드를 추척해야 합니다:
Unions are like enums, but you need to track the active field yourself:

```rust,editable
#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn main() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    // b는 정의 되지 않았습니다!
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}
```

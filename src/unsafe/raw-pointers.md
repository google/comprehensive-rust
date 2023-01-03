# Dereferencing Raw Pointers

포인트 생성은 안전합니다. 하지만 해제는 `unsafe`가 필요합니다:
> Creating pointers is safe, but dereferencing them requires `unsafe`:

```rust,editable
fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        // 만약 r1이 동시 쓰기가 되면 데이터 레이스가 발생합니다!
        *r1 = 10;  // Data race if r1 is being written concurrently!
        println!("r2 is: {}", *r2);
    }
}
```

# Calling Unsafe Functions

다음과 같은 추가 전제 조건이 있는 경우 함수나 메서드는 `unsafe` 표시할 수 있습니다:
> A function or method can be marked `unsafe` if it has extra preconditions you
> must uphold:

```rust,editable
fn main() {
    let emojis = "🗻∈🌏";
    unsafe {
        // 인덱스가 UTF-8 시퀀스 경계에 있지 않은경우 정의되지 않은 동작입니다.
        // Undefined behavior if indices do not lie on UTF-8 sequence boundaries.
        println!("{}", emojis.get_unchecked(0..4));
        println!("{}", emojis.get_unchecked(4..7));
        println!("{}", emojis.get_unchecked(7..11));
    }
}
```

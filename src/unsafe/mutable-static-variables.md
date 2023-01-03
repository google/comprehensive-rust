# Mutable Static Variables

불변 정적변수를 읽는 것은 '안전'합니다:
> It is safe to read an immutable static variable:

```rust,editable
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

하지만, 데이터 레이스가 발생할 수 있으므로 정적 가변변수를 읽고 쓰는 것은 '안전하지 않습니다':
> However, since data races can occur, it is unsafe to read and write mutable
> static variables:

```rust,editable
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    // 데이터 레이스 가능성 있음
    unsafe { COUNTER += inc; }  // Potential data race!
}

fn main() {
    add_to_counter(42);
    // 데이터 레이스 가능성 있음
    unsafe { println!("COUNTER: {}", COUNTER); }  // Potential data race!
}
```

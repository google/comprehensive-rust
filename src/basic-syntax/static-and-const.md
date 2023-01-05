# Static and Constant Variables

전역 상태(state) 정적 변수와 상수로 관리됩니다.
> Global state is managed with static and constant variables.

## `const`

컴파일 시점의 상수를 선언할 수 있습니다.
> You can declare compile-time constants:

```rust,editable
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}
```

## `static`
마찬가지로 정적 변수도 선언할 수 있습니다.
> You can also declare static variables:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

We will look at mutating static data in the [chapter on Unsafe Rust](../unsafe.md).

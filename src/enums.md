# Enums

`enum` 키워드는 열거형 타입을 생성합니다:
> The `enum` keyword allows the creation of a type which has a few
> different variants:

```rust,editable
fn generate_random_number() -> i32 {
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}
```

--- 
역주
- variants: 열거형의 요소들을 말합니다 열거형의 값은 variants중 하나여야 합니다.
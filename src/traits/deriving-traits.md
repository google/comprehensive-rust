# Deriving Traits

컴차일러가 여러가지 트레이트를 파생(derive)하도록 할 수 있습니다:
> You can let the compiler derive a number of traits:

```rust,editable
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
             if p1 == p2 { "yes" } else { "no" });
}
```

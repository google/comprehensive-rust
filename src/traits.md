# Traits
러스트는 인터페이스 처럼 트레이트가 있는 타입을 추상화 할 수 있습니다: 
> Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}
// name 없음. 고양이는 어쨋든 반응하지 않습니다...
// No name, cats won't respond to it anyway.
struct Cat;  

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Wuf, my name is {}!", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Miau!");
    }
}

fn main() {
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: String::from("Fido") }),
        Box::new(Cat),
    ];
    for pet in pets {
        pet.say_hello();
    }
}
```

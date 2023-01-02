# Closures

클로저 혹은 람다표현식은 익명타입입니다. 이들은 
[`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), 
[`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) 라는 특별한 트레이트를 구현합니다.

Closures or lambda expressions have types which cannot be named. However, they
implement special [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), and
[`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) traits:

```rust,editable
fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    let mul_5 = |x| x * 5;

    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 20));
}
```

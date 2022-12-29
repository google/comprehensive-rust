# Scopes and Shadowing

외부 스코프와 동일 스코프 범위의 변수를 가릴 수 있습니다.(변수 쉐도잉)
> You can shadow variables, both those from outer scopes and variables from the same scope:

```rust,editable
fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
```

---
역주
- after의 경우 inner scope 수명이 다해서 원래 변수인 a가 표시되는 겁니다. 
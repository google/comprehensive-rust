# Moves in Function Calls

값을 함수에 전달할때, 값은 함수 매개변수에 할당됩니다. 이것은 소유권을 이동시킵니다:
> When you pass a value to a function, the value is assigned to the function
> parameter. This transfers ownership:

```rust,editable
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name); // 역주: main에는 name의 소유권이 없어 재 전달이 불가
}
```

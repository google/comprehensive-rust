# The `Drop` Trait

`Drop`트레이트는 값이 스코프 밖으로 나갈때 실행하는 코드를 작성할 수 있습니다:
> Values which implement `Drop` can specify code to run when they go out of scope:

```rust,editable
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");
}
```

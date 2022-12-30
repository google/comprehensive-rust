# Tuple Structs

각 값의 이름이 중요하지 않다면 튜블 구조체를 사용할 수 있습니다:
> If the field names are unimportant, you can use a tuple struct:

```rust,editable
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
```
종종 단일 필드의 래퍼(wrapper)로 사용됩니다.(러스트에서 newtypes 패턴[^역주1]이라 부릅니다):
> This is often used for single-field wrappers (called newtypes):

```rust,editable,compile_fail
struct PoundOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}

```
---
역주

[^역주1]: 값 자체가 의미를 가지는 경우(cm, mm, kg 등)에 이를 표기하기 위해 단일 값을 래핑하는 패턴입니다.(예시는 물리 단위(뉴턴, 파운드(힘))를 래핑하고 있습니다. )

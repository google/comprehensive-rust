# Borrowing

함수 호출시 값의 소유권을 이동하는 대신의 함수가 값을 _빌려_올 수 있습니다:
> Instead of transferring ownership when calling a function, you can let a
> function _borrow_ the value:

```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}
```

* `add` 함수는 두 Point객체 값을 _빌려_오와서 새로운 Point객체를 반환합니다.
* 호출자(main 함수)는 여전히 p1, p2의 소유권을 유지합니다.
> * The `add` function _borrows_ two points and returns a new point.
> * The caller retains ownership of the inputs.

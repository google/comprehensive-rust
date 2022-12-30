# Lifetimes in Function Calls

함수는 인수를 빌리는 것 외에도 빌린 값을 반환할 수 있습니다:
> In addition to borrowing its arguments, a function can return a borrowed value:

```rust,editable
#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);  // Put into different scope
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}
```

* `'a`는 제너릭 매개변수로 컴파일러로에 의해 추론됩니다.
* 수명은 `'` 로 표기하며 러스트에서는 일반적으로 `'a` 라고 표현합니다.
* `&'a Point` 는 as  `a`와 동일한 수명을 가지는 빌려온 `Point` 입니다.
  * **중요** : 다른 스코프에 있어도 적용 됩니다.
> * `'a` is a generic parameter, it is inferred by the compiler.
> * Lifetimes start with `'` and `'a` is a typical default name.
> * Read `&'a Point` as "a borrowed `Point` which is valid for at least the lifetime `a`".
> * The _at least_ part is important when parameters are in different scopes.

--- 
역주
- left_most함수가 종료되더라도 리턴값 p3은 매개변수인 p1,p2와 동일한 수명이라 main에서도 유효해집니다. 
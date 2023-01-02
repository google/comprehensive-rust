# Niche Optimization

```rust,editable
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```
`Box`는 비어있을 수 없습니다. 따라서 포인트는 항상 유효하고 `null`이 아닙니다.
이는 컴파일러가 메모리 레이아웃을 최적화 할 수 있게 해줍니다.
> A `Box` cannot be empty, so the pointer is always valid and non-`null`. This
> allows the compiler to optimize the memory layout:

```bob
 Stack                           Heap
.- - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                         :     :                                               :
:    list                 :     :                                               :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:   | 0      | 1     |    :     : .->| 0      |  2     | .->| ////// | //// |   :
:   | "1/Tag"| o-----+----+-----+-'  | "1/Tag"|  o-----+-'  | "1/Tag"| null |   :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:                         :     :                                               :
:                         :     :                                               :
`- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - - -'
```

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

A `Box` cannot be empty, so the pointer is always valid and non-`null`. This
allows the compiler to optimize the memory layout:

```bob
 Stack                           Heap
.- - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - -.
:                         :     :                                             :
:    list                 :     :                                             :
:   +----+----+           :     :    +----+----+    +----+------+             :
:   | 1  | o--+-----------+-----+--->| 2  | o--+--->| // | null |             :
:   +----+----+           :     :    +----+----+    +----+------+             :
:                         :     :                                             :
:                         :     :                                             :
`- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - -'
```

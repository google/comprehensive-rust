# Box with Recursive Data Structures

Recursive data types or data types with dynamic sizes need to use a `Box`:

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

```bob
 Stack                           Heap
.- - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                         :     :                                               :
:    list                 :     :                                               :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:   | Tag    | Cons  |    :     : .->| Tag    | Cons   | .->| Tag    | Nil  |   :
:   | 0      | 1     |    :     : |  | 0      | 2      | |  | ////// | //// |   :
:   | 1      | o-----+----+-----+-'  | 1      | o------+-'  | ////// | //// |   :
:   +--------+-------+    :     :    +--------+--------+    +--------+------+   :
:                         :     :                                               :
:                         :     :                                               :
`- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - - -'
```


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
:   +------+----+----+    :     :    +------+----+----+    +------+----+----+   :
:   | Cons | 1  | o--+----+-----+--->| Cons | 2  | o--+--->| Nil  | // | // |   :
:   +------+----+----+    :     :    +------+----+----+    +------+----+----+   :
:                         :     :                                               :
:                         :     :                                               :
'- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - - -'
```

<details>

* If the `Box` was not used here and we attempted to embed a `List` directly into the `List`,
the compiler would not compute a fixed size of the struct in memory, it would look infinite.

* `Box` solves this problem as it has the same size as a regular pointer and just points at the next
element of the `List` in the heap.

* Remove the `Box` in the List definition and show the compiler error. "Recursive with indirection" is a hint you might want to use a Box or reference of some kind, instead of storing a value directly.   
    
</details>

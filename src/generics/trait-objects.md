# Trait Objects

We've seen how a function can take arguments which implement a trait:

```rust,editable
use std::fmt::Display;

fn print<T: Display>(x: T) {
    println!("Your value: {x}");
}

fn main() {
    print(123);
    print("Hello");
}
```

However, how can we store a collection of mixed types which implement `Display`?

```rust,editable,compile_fail
fn main() {
    let xs = vec![123, "Hello"];
}
```

For this, we need _trait objects_:

```rust,editable
use std::fmt::Display;

fn main() {
    let xs: Vec<Box<dyn Display>> = vec![Box::new(123), Box::new("Hello")];
    for x in xs {
        println!("x: {x}");
    }
}
```

Memory layout after allocating `xs`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                           :     :                                               :
:    xs                     :     :                                               :
:   +-----------+-------+   :     :   +-----+-----+                               :
:   | ptr       |   o---+---+-----+-->| o o | o o |                               :
:   | len       |     2 |   :     :   +-|-|-+-|-|-+                               :
:   | capacity  |     2 |   :     :     | |   | |   +----+----+----+----+----+    :
:   +-----------+-------+   :     :     | |   | '-->| H  | e  | l  | l  | o  |    :
:                           :     :     | |   |     +----+----+----+----+----+    :
`- - - - - - - - - - - - - -'     :     | |   |                                   :
                                  :     | |   |     +-------------------------+   :
                                  :     | |   '---->| "<str as Display>::fmt" |   :
                                  :     | |         +-------------------------+   :
                                  :     | |                                       :
                                  :     | |   +----+----+----+----+               :
                                  :     | '-->| 7b | 00 | 00 | 00 |               :
                                  :     |     +----+----+----+----+               :
                                  :     |                                         :
                                  :     |     +-------------------------+         :
                                  :     '---->| "<i32 as Display>::fmt" |         :
                                  :           +-------------------------+         :
                                  :                                               :
                                  :                                               :
                                  '- - - - - - - - - - - - - - - - - - - - - - - -'
```

Similarly, you need a trait object if you want to return different values
implementing a trait:

```rust,editable
fn numbers(n: i32) -> Box<dyn Iterator<Item=i32>> {
    if n > 0 {
        Box::new(0..n)
    } else {
        Box::new((n..0).rev())
    }
}

fn main() {
    println!("{:?}", numbers(-5).collect::<Vec<_>>());
    println!("{:?}", numbers(5).collect::<Vec<_>>());
}

```

# Stack Memory

Creating a `String` puts fixed-sized data on the stack and dynamically sized
data on the heap:

```rust,editable
fn main() {
    let s1 = String::from("Hello");
}
```

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - -.
:                           :     :                               :
:    s1                     :     :                               :
:   +-----------+-------+   :     :   +----+----+----+----+----+  :
:   | ptr       |   o---+---+-----+-->| H  | e  | l  | l  | o  |  :
:   | len       |     5 |   :     :   +----+----+----+----+----+  :
:   | capacity  |     5 |   :     :                               :
:   +-----------+-------+   :     :                               :
:                           :     `- - - - - - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

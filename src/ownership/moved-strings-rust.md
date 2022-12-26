# Moved Strings in Rust

```rust,editable
fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1;
}
```

- The heap data from `s1` is reused for `s2`.
- When `s1` goes out of scope, nothing happens (it has been moved from).

Before move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1                     :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+-----+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :     :   +----+----+----+----+   :
:   | capacity  |     4 |   :     :                           :
:   +-----------+-------+   :     :                           :
:                           :     `- - - - - - - - - - - - - -'
:                           :
`- - - - - - - - - - - - - -'
```

After move to `s2`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1 "(inaccessible)"    :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+--+--+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :  |  :   +----+----+----+----+   :
:   | capacity  |     4 |   :  |  :                           :
:   +-----------+-------+   :  |  :                           :
:                           :  |  `- - - - - - - - - - - - - -'
:    s2                     :  |
:   +-----------+-------+   :  |
:   | ptr       |   o---+---+--'
:   | len       |     4 |   :
:   | capacity  |     4 |   :
:   +-----------+-------+   :
:                           :
`- - - - - - - - - - - - - -'
```

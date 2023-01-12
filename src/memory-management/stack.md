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

<details>

* Mention that a `String` is backed by a `Vec`, so it has a capacity and length and can grow if mutable via reallocation on the heap.

* If students ask about it, you can mention that the underlying memory is heap allocated using the [System Allocator] and custom allocators can be implemented using the [Allocator API]

</details>

[System Allocator]: https://doc.rust-lang.org/std/alloc/struct.System.html
[Allocator API]: https://doc.rust-lang.org/std/alloc/index.html

# Stack Memory

`String` 타입은 힙에 동적으로 데이터를 저장하고 크기 고정된 데이터(힙 데이터에 대한 정보)를 스택에 저장합니다.
> Creating a `String` puts fixed-sized data on the stack and dynamically sized
> data on the heap:

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

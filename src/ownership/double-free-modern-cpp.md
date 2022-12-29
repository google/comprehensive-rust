# Double Frees in Modern C++

Modern C++은 이 문제를 다르게 해결합니다:
> Modern C++ solves this differently:

```c++
std::string s1 = "Cpp";
std::string s2 = s1;  // Duplicate the data in s1.
```

* `s1`의 힙 데이터는 복제되고, `s2`는 독립적인 복사본을 얻습니다.
* `s1` 와 `s2`의 스코프가 종료되면 각각의 메모리가 해제됩니다.
> * The heap data from `s1` is duplicated and `s2` gets its own independent copy.
> * When `s1` and `s2` go out of scope, they each free their own memory.

Before copy-assignment:


```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

After copy-assignment:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - -.
:                           :     :                       :
:    s1                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+--+--+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     :                       :
:    s2                     :     :                       :
:   +-----------+-------+   :     :   +----+----+----+    :
:   | ptr       |   o---+---+-----+-->| C  | p  | p  |    :
:   | len       |     3 |   :     :   +----+----+----+    :
:   | capacity  |     3 |   :     :                       :
:   +-----------+-------+   :     :                       :
:                           :     `- - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

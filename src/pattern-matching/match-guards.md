# Match Guards

match를 사용할때 각 패턴에 대해서 불리언으로 반환되는 검사식(guard)를 추가할 수 있습니다. 
> When matching, you can add a _guard_ to a pattern. This is an arbitrary Boolean
> expression which will be executed if the pattern matches:

```rust,editable
{{#include ../../third_party/rust-by-example/match-guards.rs}}
```

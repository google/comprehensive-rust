# Variant Payloads

You can define richer enums where the variants carry data. You can then use the
`match` statement to extract the data from each variant:

```rust,editable
{{#include ../../third_party/rust-by-example/webevent.rs}}
```

<details>

Key Points:
  
* This might be the first time seeing pattern matching so perhaps introduce the topic, more details are provided in further slides.
  * A match expression can return any type.
  * In a match, we check some expressions against some possible options, called match arms. Each option is declared with a pattern, an arrow, some code and then a comma. When it's a multi-line block we use a pair curly braces.
  * Starting from the top we look for what pattern matches the value then run the code following the arrow. Once we find a match, we stop. 
  * You might demonstrate what happens when the search is inexhaustive. Note the advantage the Rust compiler provides by confirming when all cases are handled. 
* In the above example, accessing the `char` in `KeyPress`, or `x` and `y` in `Click` only works within a `match` statement.
* `match` inspects a hidden discriminant field in the `enum`.
* `WebEvent::Click { ... }` is not exactly the same as `WebEvent::Click(Click)` with a top level `struct Click { ... }`. The inlined version cannot implement traits, for example.  
  
</details>

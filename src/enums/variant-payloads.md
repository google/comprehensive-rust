# Variant Payloads

You can define richer enums where the variants carry data. You can then use the
`match` statement to extract the data from each variant:

```rust,editable
{{#include ../../third_party/rust-by-example/webevent.rs}}
```

<details>

* In the above example, accessing the `char` in `KeyPress`, or `x` and `y` in `Click` only works within a `match` statement.
* `match` inspects a hidden discriminant field in the `enum`.
* `WebEvent::Click { ... }` is not exactly the same as `WebEvent::Click(Click)` with a top level `struct Click { ... }`. The inlined version cannot implement traits, for example.

</details>

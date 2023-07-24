# Error handling for imported Javascript methods

Javascript methods that throw can be imported in Rust like this:

```javascript
window.throwsIfPair = (num) => {
  if (num % 2 === 0) throw new Error("Pair number");
  return num;
};
```

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn throwsIfPair(text: i32) -> Result<i32, JsValue>;
}
```

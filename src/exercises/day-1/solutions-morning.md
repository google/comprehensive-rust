# Day 1 Morning Exercises

## Arrays and `for` Loops

([back to exercise](for-loops.md))

```rust
{{#include for-loops.rs}}
```
### Bonus question

A two-dimensional slice-of-slices would be fine for argument if you do something like

```rust
{{#include for-loops-bonus.rs}}
```

However, it won't work very well for a return type. Since the `transpose` function need to allocate new data to hold the output value, it will go out of scope at the end of `transpose` so you need to return the data itself instead of its reference.
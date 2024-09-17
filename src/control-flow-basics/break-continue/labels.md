# Labels

Both `continue` and `break` can optionally take a label argument which is used
to break out of nested loops:

```rust,editable
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}
```

<details>

- Labeled break also works on arbitrary blocks, e.g.
  ```rust
  'label: {
      break 'label;
      println!("This line gets skipped");
  }
  ```

</details>

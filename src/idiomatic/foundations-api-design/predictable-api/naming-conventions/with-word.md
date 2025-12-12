---
minutes: 2
---

# `with` in normal use

Sometimes a `with` is just a `with`.

`with` when used in common English contexts.

```rust
// impl block for slices
impl <T> [T] {
    // A condition, but doesn't start with `is`, and uses `with` as a normal word.
    fn starts_with(&self, &[T]) -> bool;
}
```

<details>

- Name fragments are not hard rules, they are guidance. Sometimes a method's
  name will include words that break its pattern.

- In this example with have `starts_with`, which is a boolean condition that
  does not start with "is" and is suffixed by "with".

  If naming conventions were to be treated as hard rules, this would fail as a
  case.

  This is a good name for understanding what is going on at the callsite. We end
  up writing `<varible>.starts_with(<sequence>)` which scans well for authors
  and readers of code.

- Remember: the point of naming conventions is predictability, and how
  predictability is in service of callsite clarity and readability.

</details>

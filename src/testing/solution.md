# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

The solution addresses several bugs in the original implementation:

- **Whitespace:** We specifically check for and ignore whitespace using
  `c.is_whitespace()`.
- **Invalid Characters:** We return `false` immediately if we encounter any
  character that is not a digit and not whitespace. The original code simply
  ignored them (via `else { continue }`), effectively treating "123a4" as
  "1234".
- **Length Check:** We track the number of digits seen (`digits`) and return
  `false` if fewer than 2 digits are found.
- **Unit Tests:** We add tests for edge cases: empty strings, strings with only
  whitespace, single-digit strings, and strings with invalid characters. This
  ensures the validation logic is robust.

<details>

- Note that `c.to_digit(10)` returns `Some(digit)` if `c` is a base-10 digit,
  and `None` otherwise.
- The condition `digits >= 2 && sum % 10 == 0` ensures both requirements are
  met.

</details>

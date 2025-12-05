---
minutes: 2
---

# Owned

Suffix for functions and methods about owned values in a context where
references are common.

```
String::to_owned
Cow::into_owned
```

<details>
- Most often seen in to_owned methods, especially on strings.

Some exceptions, such as Cow (copy on write) having an `into_owned` method that
consumes the Cow and produces an owned value via the `ToOwned` trait.

</details>

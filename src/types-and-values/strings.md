---
minutes: 10
---

# Strings

Rust has two types to represent strings, both of which will be covered in more
depth later. Both _always_ store UTF-8 encoded strings.

- `String` - a modifiable, owned string.
- `&str` - a read-only string. String literals have this type.

```rust,editable
fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    //println!("{:?}", &sentence[12..13]);
}
```

<details>

This slide introduces strings. Everything here will be covered in more depth
later, but this is enough for subsequent slides and exercises to use strings.

- Invalid UTF-8 in a string is UB, and this not allowed in safe Rust.

- `String` is a user-defined type with a constructor (`::new()`) and methods
  like `s.push_str(..)`.

- The `&` in `&str` indicates that this is a reference. We will cover references
  later, so for now just think of `&str` as a unit meaning "a read-only string".

- The commented-out line is indexing into the string by byte position. `12..13`
  does not end on a character boundary, so the program panics. Adjust it to a
  range that does, based on the error message.

- Raw strings allow you to create a `&str` value with escapes disabled:
  `r"\n" == "\\n"`. You can embed double-quotes by using an equal amount of `#`
  on either side of the quotes:

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  fn main() {
      println!(r#"<a href="link.html">link</a>"#);
      println!("<a href=\"link.html\">link</a>");
  }
  ```

- Using `{:?}` is a convenient way to print array/vector/struct of values for
  debugging purposes, and it's commonly used in code.

</details>

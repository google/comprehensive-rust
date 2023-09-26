# Scalar Types

|                        | Types                                      | Literals                       |
|------------------------|--------------------------------------------|--------------------------------|
| Signed integers        | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123_i64` |
| Unsigned integers      | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10_u16`           |
| Floating point numbers | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2_f32`    |
| Strings                | `&str`                                     | `"foo"`, `"two\nlines"`        |
| Unicode scalar values  | `char`                                     | `'a'`, `'α'`, `'∞'`            |
| Booleans               | `bool`                                     | `true`, `false`                |

The types have widths as follows:

* `iN`, `uN`, and `fN` are _N_ bits wide,
* `isize` and `usize` are the width of a pointer,
* `char` is 32 bits wide,
* `bool` is 8 bits wide.

<details>

There are a few syntaxes which are not shown above:

- Raw strings allow you to create a `&str` value with escapes disabled: `r"\n"
  == "\\\\n"`. You can embed double-quotes by using an equal amount of `#` on
  either side of the quotes:

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  fn main() {
      println!(r#"<a href="link.html">link</a>"#);
      println!("<a href=\"link.html\">link</a>");
  }
  ```

- Byte strings allow you to create a `&[u8]` value directly:

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  fn main() {
      println!("{:?}", b"abc");
      println!("{:?}", &[97, 98, 99]);
  }
  ```

- All underscores in numbers can be left out, they are for legibility only.
  So `1_000` can be written as `1000` (or `10_00`), and `123_i64` can be written as `123i64`.

</details>

# Additional Types

| Rust Type         | C++ Type             |
| ----------------- | -------------------- |
| `String`          | `rust::String`       |
| `&str`            | `rust::Str`          |
| `CxxString`       | `std::string`        |
| `&[T]`/`&mut [T]` | `rust::Slice`        |
| `Box<T>`          | `rust::Box<T>`       |
| `UniquePtr<T>`    | `std::unique_ptr<T>` |
| `Vec<T>`          | `rust::Vec<T>`       |
| `CxxVector<T>`    | `std::vector<T>`     |

<details>

- These types can be used in the fields of shared structs and the arguments and
  returns of extern functions.
- Note that Rust's `String` does not map directly to `std::string`. There are a
  few reasons for this:
  - `std::string` does not uphold the UTF-8 invariant that `String` requires.
  - The two types have different layouts in memory and so can't be passed
    directly between languages.
  - `std::string` requires move constructors that don't match Rust's move
    semantics, so a `std::string` can't be passed by value to Rust.

</details>

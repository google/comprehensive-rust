# Array Types

The array types (`T[]`, `byte[]`, and `List<T>`) get translated to the
appropriate Rust array type depending on how they are used in the function
signature:

| Position               | Rust Type     |
|------------------------|---------------|
| `in` argument          | `&[T]`        |
| `out`/`inout` argument | `&mut Vec<T>` |
| Return                 | `Vec<T>`      |

<details>

* In Android 13 or higher, fixed-size arrays are supported, i.e. `T[N]` becomes
  `[T; N]`. Fixed-size arrays can have multiple dimensions (e.g. int[3][4]). In
  the Java backend, fixed-size arrays are represented as array types.

</details>

# Primitive Types

Primitive types map (mostly) idiomatically:

| AIDL Type | Rust Type | Note                                |
|-----------|-----------|-------------------------------------|
| `boolean` | `bool`    |                                     |
| `byte`    | `i8`      | Note that bytes are signed.         |
| `char`    | `u16`     | Note the usage of `u16`, NOT `u32`. |
| `int`     | `i32`     |                                     |
| `long`    | `i64`     |                                     |
| `float`   | `f32`     |                                     |
| `double`  | `f64`     |                                     |
| `String`  | `String`  |                                     |

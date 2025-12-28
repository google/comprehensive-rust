---
minutes: 2
---

# `from`

A constructor function, strongly implying "type conversion".

```rust,compile_fail
impl CStr {
    unsafe fn from_ptr<'a>(ptr: *const i8) -> &'a CStr;
}

impl Duration {
    fn from_days(days: u64) -> Duration;
}

impl<T> Vec<T> {
    fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>;
}

impl i32 {
    fn from_ascii(src: &[u8]) -> Result<i32, ParseIntError>;
}

impl u32 {
    fn from_le_bytes(bytes: [u8; 4]) -> u32;
}
```

<details>
- Prefix for constructor-style, `From`-trait-style functions.

- These functions can take multiple arguments, but usually imply the user is
  doing more of the work than a usual constructor would.

  `new` is still preferred for most constructor-style functions, the implication
  for `from` is transformation of one data type to another.

- Ask: Without looking at the standard library documentation, what would the
  argument type of `u32::from_be` be?

  Answer guidance: we already see `u32::from_le_bytes` on the slide, it takes a
  slice of bytes. So from_le must be simpler, taking not bytes. Think about the
  contrast between `u32` and `be`. The argument must be a big-endian `u32`!

  Follow-up question: How about `str::from_utf8`?

  Answer guidance: `str` vs `utf8`. The argument can't be a `str` because every
  `str` is valid UTF-8. So what is the simplest way to provide UTF-8 data? A
  slice of bytes.

  Follow-up: Why not `str::from_utf8_bytes`?

  Answer: It could be in theory. However, the "omit needless words" principle
  applies, the word "bytes" would merely repeat the obvious - could a UTF-8
  sequence ever be non-bytes?

</details>

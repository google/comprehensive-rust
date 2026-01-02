# Getter example

```rust,editable
/// Return the element at `index` from `arr`
unsafe fn get(arr: *const i32, index: usize) -> i32 {
    unsafe { *arr.add(index) }
}
```

<details>

“Safety preconditions are conditions on code that must be satisfied to maintain
Rust's safety guarantees

“You're likely to see a strong affinity between safety preconditions and the
rules of Safe Rust.”

Ask: “What are the safety preconditions of `get`?”

- The pointer `arr` is non-null, well-aligned and refers to an array of `i32`
- `index` is in-bounds

Add safety comments:

```rust,ignore
/// Return the element at `index` from `arr`
///
/// # Safety
///
/// - `arr` is non-null, correctly aligned and points to a valid `i32`
/// - `index` is in-bounds for the array
unsafe fn get(arr: *const i32, index: usize) -> i32 {
    // SAFETY: Caller guarantees that index is inbounds
    unsafe { *arr.add(index) }
}
```

Optional: Add runtime checks can be added in debug builds to provide some extra
robustness.

```rust,ignore
debug_assert!(!arr.is_null());
debug_assert_eq!(arr as usize % std::mem::align_of::<i32>(), 0);
```

</details>

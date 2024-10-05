---
minutes: 5
---

# Calling Unsafe Functions

Failing to uphold the safety requirements breaks memory safety!

```rust,editable
#[derive(Debug)]
#[repr(C)]
struct KeyPair {
    pk: [u16; 4], // 8 bytes
    sk: [u16; 4], // 8 bytes
}

const PK_BYTE_LEN: usize = 8;

fn log_public_key(pk_ptr: *const u16) {
    let pk: &[u16] = unsafe { std::slice::from_raw_parts(pk_ptr, PK_BYTE_LEN) };
    println!("{pk:?}");
}

fn main() {
    let key_pair = KeyPair { pk: [1, 2, 3, 4], sk: [0, 0, 42, 0] };
    log_public_key(key_pair.pk.as_ptr());
}
```

Always include a safety comment for each `unsafe` block. It must explain why the
code is actually safe. This example is missing a safety comment and has UB.

<details>

Key points:

- The second argument to `slice::from_raw_parts` is the number of _elements_,
  not bytes! This example demonstrate undefined behavior by reading past the end
  of one array and into another.

- The standard library contain many low-level unsafe functions. Prefer the safe
  alternatives when possible!

- If you use an unsafe function as an optimization, make sure to add a benchmark
  to demonstrate the gain.

</details>

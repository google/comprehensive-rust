---
minutes: 0
---

# Single-use values

In some circumstances we want values that can be used _exactly once_. One critical example of this is in cryptography, one-use values "Nonces"

```rust,editable
fn main() {
pub struct Key;

// Pretend this is a cryptographically unique, use-once number.
pub struct Nonce(u32);

unsafe fn new_nonce_from_raw(nonce: u32) -> Nonce {
    Nonce(nonce)
}

let nonce = unsafe { new_nonce_from_raw(1337) };
let data = [1u8, 2u8, 3u8, 4u8];
let key = Key;

// The key and data can be re-used, copied, etc. but the nonce cannot.
fn encrypt(nonce: Nonce, key: &Key, data: &[u8]) {}

encrypt(nonce, &key, &data);
encrypt(nonce, &key, &data); // 🛠️❌
}
```
<details>

- Owned "consumption" lets us model use-once values.

- Not implementing clone/copy here & making the interior type opaque (as per the newtype pattern) is _intentional_, as it prevents multiple uses of the same, API-controlled value.

- A Nonce is a additional piece of random, unique data during an encryption process that helps prevent "replay attacks".

- TODO: put a reference to how this is used in the Real World in rust cryptography.

</details>
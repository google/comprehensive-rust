---
minutes: 0
---

# Single-use values

In some circumstances we want values that can be used _exactly once_. One critical example of this is in cryptography: "Nonces."

```rust,editable
fn main() {
pub struct Key;

// Pretend this is a cryptographically unique, use-once number.
pub struct Nonce(u32);

// It's unsafe to declare a nonce directly! In practice,
// this would be done with an RNG source, and potentially
// a timestamp.
unsafe fn new_nonce_from_raw(nonce: u32) -> Nonce {
    Nonce(nonce)
}

let nonce = unsafe { new_nonce_from_raw(1337) };
let data_1: [u8; 4] = [1, 2, 3, 4];
let data_2: [u8; 4] = [4, 3, 2, 1];
let key = Key;

// The key and data can be re-used, copied, etc. but the nonce cannot.
fn encrypt(nonce: Nonce, key: &Key, data: &[u8]) {}

encrypt(nonce, &key, &data_1);
encrypt(nonce, &key, &data_2); // 🛠️❌
}
```
<details>

- Owned "consumption" lets us model single-once values.

- Not implementing clone/copy here & making the interior type opaque (as per the newtype pattern) is _intentional_, as it prevents multiple uses of the same, API-controlled value.

- I.e. A Nonce is a additional piece of random, unique data during an encryption process that helps prevent "replay attacks".

- In practice people have ended up re-using nonces in circumstances where security is important, making it possible for private key information to be derived by attackers.

- By tying nonce creation and consumption up in rust's ownership model, and by not implementing clone/copy on sensitive single-use data, we can prevent this kind of dangerous misuse.

- Cryptography Nuance: There is still the case where a nonce may be used twice if it's created through purely a pseudo-random process with no additional metadata, and that circumstance can't be avoided through this particular method. This kind of API prevents one kind of misuse, but not all kinds.

</details>
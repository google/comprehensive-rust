---
minutes: 5
---

# Single-use values

In some circumstances we want values that can be used _exactly once_. One
critical example of this is in cryptography: "Nonces."

```rust,editable
mod cryptography {
    pub struct Key(/* specifics omitted */);
    // Pretend this is a cryptographically sound, single-use number.
    pub struct Nonce(u32);
    // And pretend this is cryptographically sound random generator function.
    pub fn new_nonce() -> Nonce {
        Nonce(std::time::UNIX_EPOCH.elapsed().unwrap_or_default().subsec_nanos())
    }

    // We consume a nonce, but not the key or the data.
    pub fn encrypt(nonce: Nonce, key: &Key, data: &[u8]) {}
}

use cryptography::*;
fn main() {
    let nonce = new_nonce();
    let data_1: [u8; 4] = [1, 2, 3, 4];
    let data_2: [u8; 4] = [4, 3, 2, 1];
    let key = Key(/* specifics omitted */);

    // The key and data can be re-used, copied, etc. but the nonce cannot.
    encrypt(nonce, &key, &data_1);
    // encrypt(nonce, &key, &data_2); // 🛠️❌
}
```

<details>

- Owned "consumption" of values lets us model things that need to be single-use.

- By keeping constructors private and not implementing clone/copy for a type,
  making the interior type opaque (as per the newtype pattern), we can prevent
  multiple uses of the same value while keeping control of how that value is
  bui.

- A nonce is a piece of random, unique data used in cryptographic protocols to
  prevent replay attacks.

  - In practice people have ended up accidentally re-using nonces. Most
    commonly, this causes the cryptographic protocol to completely break down
    and stop fulfilling its function. Depending on the specifics of nonce reuse
    and cryptography at hand, private keys can also become computable by
    attackers.

  - By tying nonce creation and consumption up in rust's ownership model, and by
    not implementing clone/copy on sensitive single-use data, we can prevent
    this kind of dangerous misuse.

  - Cryptography Nuance: There is still the case where a nonce may be used twice
    if it's created through purely a pseudo-random process with no additional
    metadata, and that circumstance can't be avoided through this particular
    method. This kind of API prevents one kind of misuse, but not all kinds.

</details>

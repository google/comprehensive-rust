---
minutes: 10
---

# Single-use values

In some circumstances we want values that _can only be used once_. One critical
example of this is in cryptography: "Nonces."

```rust,editable
pub struct Key(/* specifics omitted */);
// A single-use number suitable for cryptographic purposes.
pub struct Nonce(u32);
// A cryptographically sound random generator function.
pub fn new_nonce() -> Nonce {
    Nonce(4) // chosen by a fair dice roll, https://xkcd.com/221/
}
// Consume a nonce, but not the key or the data.
pub fn encrypt(nonce: Nonce, key: &Key, data: &[u8]) {}

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

- What if we want to be able to guarantee that a value can only be used once?

- Motivation: A nonce is a piece of random, unique data used in cryptographic
  protocols to prevent replay attacks.

  Background: In practice people have ended up accidentally re-using nonces.
  Most commonly, this causes the cryptographic protocol to completely break down
  and stop fulfilling its function. Depending on the specifics of nonce reuse
  and cryptography at hand, private keys can also become computable by
  attackers.

- Rust has an obvious tool for "Once you use this, you can't use it anymore":
  Using a value as an _owned argument_.

- Highlight: the `encrypt` function takes references for `key` and `data` but
  not `nonce`

- By keeping constructors private and not implementing clone/copy for a type,
  making the interior type opaque (as per the newtype pattern), we can prevent
  multiple uses of the same value.

- Ask: What are we missing from the newtype pattern in the slide's code?

  Expect: Module boundary.

  Demonstrate: Without a module boundary a user can construct a nonce on their
  own.

  Fix: Put `Key`, `Nonce`, and `new_nonce` behind a module.

- Cryptography Nuance: There is still the case where a nonce may be used twice
  if it's created through purely a pseudo-random process with no additional
  metadata, and that circumstance can't be avoided through this particular
  method. This API design prevents one kind of misuse, but not all kinds.

</details>

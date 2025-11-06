---
minutes: 10
---

# Single-use values

Sometimes we want values that _can only be used once_. One critical example of
this is in cryptography: A "Nonce."

```rust,editable
pub struct Key(/* specifics omitted */);
/// A single-use number suitable for cryptographic purposes.
pub struct Nonce(u32);
/// A cryptographically sound random generator function.
pub fn new_nonce() -> Nonce {
    Nonce(4) // chosen by a fair dice roll, https://xkcd.com/221/
}
/// Consume a nonce, but not the key or the data.
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

- Problem: How can we guarantee a value is used only once?

- Motivation: A nonce is a piece of random, unique data used in cryptographic
  protocols to prevent replay attacks.

  Background: In practice people have ended up accidentally re-using nonces.
  Most commonly, this causes the cryptographic protocol to completely break down
  and stop fulfilling its function.

  Depending on the specifics of nonce reuse and cryptography at hand, private
  keys can also become computable by attackers.

- Rust has an obvious tool for achieving the invariant "Once you use this, you
  can't use it again": passing a value as an _owned argument_.

- Highlight: the `encrypt` function takes `nonce` by value (an owned argument),
  but `key` and `data` by reference.

- The technique for single-use values is as follows:

  - Keep constructors private, so a user can't construct values with the same
    inner value twice.

  - Don't implement `Clone`/`Copy` traits or equivalent methods, so a user can't
    duplicate data we want to keep unique.

  - Make the interior type opaque (like with the newtype pattern), so the user
    cannot modify an existing value on their own.

- Ask: What are we missing from the newtype pattern in the slide's code?

  Expect: Module boundary.

  Demonstrate: Without a module boundary a user can construct a nonce on their
  own.

  Fix: Put `Key`, `Nonce`, and `new_nonce` behind a module.

## More to Explore

- Cryptography Nuance: A nonce might still be used twice if it was created
  through pseudo-random process with no actual randomness. That can't be
  prevented through this method. This API design prevents one nonce duplication,
  but not all logic bugs.

</details>

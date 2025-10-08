---
minutes: 10
---

# Variable-Specific Tokens (Branding 1/4)

What if we want to tie a token to a specific variable?

```rust,editable
struct Bytes {
    bytes: Vec<u8>,
}
struct ProvenIndex(usize);

impl Bytes {
    fn get_index(&self, ix: usize) -> Option<ProvenIndex> {
        if ix < self.bytes.len() { Some(ProvenIndex(ix)) } else { None }
    }
    fn get_proven(&self, token: &ProvenIndex) -> u8 {
        self.bytes[token.0]
    }
}

fn main() {
    let data_1 = Bytes { bytes: vec![0, 1, 2] };
    if let Some(token_1) = data_1.get_index(2) {
        data_1.get_proven(&token_1); // Works fine!
        let data_2 = Bytes { bytes: vec![0, 1] };
        data_2.get_proven(&token_1); // Panics! How do we prevent this at compile time?
    }
}
```

<details>

- What if we want to tie a token to a _specific variable_ in our code? Can we do
  this in Rust's type system?

- Motivation: We want to have a Token Type that represents a known, valid index
  for a a byte array that we know exists.

  In this example there's nothing stopping the proven index of one array being
  used on a different array.

  The code here panics! We want to prevent this "crossover" of token types for
  indexes at compile time.

- Ask: How might we try to do this?

  Expect students to not reach a good implementation from this, but be willing
  to experiment and follow through on suggestions.

- Ask: What are the alternatives, why are they not good enough?

  Expect runtime checking of index bounds, especially as `get_index` already
  uses runtime checking.

  Runtime checking of bounds does not prevent the erroneous crossover in the
  first place, only the `panic` result. That erroneous checking

- The kind of token-association we will be doing here is called Branding. Doing
  this lets us expand the "proof of work from elsewhere" to more general aspects
  of rust.

- [`GhostCell`](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf) is a
  prominent user of this, later slides will touch on it.

</details>

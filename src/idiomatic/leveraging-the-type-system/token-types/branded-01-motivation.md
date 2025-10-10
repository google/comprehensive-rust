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
        unsafe { *self.bytes.get_unchecked(token.0) }
    }
}

fn main() {
    let data_1 = Bytes { bytes: vec![0, 1, 2] };
    if let Some(token_1) = data_1.get_index(2) {
        data_1.get_proven(&token_1); // Works fine!

        // let data_2 = Bytes { bytes: vec![0, 1] };
        // data_2.get_proven(&token_1); // Panics! Can we prevent this?
    }
}
```

<details>

- What if we want to tie a token to a _specific variable_ in our code? Can we do
  this in Rust's type system?

- Motivation: We want to have a Token Type that represents a known, valid index
  into a byte array.

  Once we have these proven indexes we would be able to avoid bounds checks
  entirely, as the tokens would act as the _proof of an existing index_.
  
  Since the index is known to be valid, `get_proven()` can skip the bounds check.

  In this example there's nothing stopping the proven index of one array being
  used on a different array. If an index is out of bounds in this case, it is
  undefined behavior.

- Demonstrate: Uncomment the `data_2.get_proven(&token_1);` line.

  The code here panics! We want to prevent this "crossover" of token types for
  indexes at compile time.

- Ask: How might we try to do this?

  Expect students to not reach a good implementation from this, but be willing
  to experiment and follow through on suggestions.

- Ask: What are the alternatives, why are they not good enough?

  Expect runtime checking of index bounds, especially as both `Vec::get` and
  `Bytes::get_index` already uses runtime checking.

  Runtime bounds checking does not prevent the erroneous crossover in the first
  place, it only guarantees a panic.

- The kind of token-association we will be doing here is called Branding. This
  is an advanced technique that expands applicability of token types to more API
  designs.

- [`GhostCell`](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf) is a
  prominent user of this, later slides will touch on it.

</details>

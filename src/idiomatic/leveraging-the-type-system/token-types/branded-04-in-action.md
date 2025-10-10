---
minutes: 15
---

# Branded Types in Action (Branding 4/4)

```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);
struct ProvenIndex<'id>(usize, InvariantLifetime<'id>);

struct Bytes<'id>(Vec<u8>, InvariantLifetime<'id>);

impl<'id> Bytes<'id> {
    fn new<T>(
        // The data we want to modify in this context.
        bytes: Vec<u8>,
        // The function that uniquely brands the lifetime of a `Bytes`
        f: impl for<'a> FnOnce(Bytes<'a>) -> T,
    ) -> T {
        f(Bytes(bytes, InvariantLifetime::default()))
    }

    fn get_index(&self, ix: usize) -> Option<ProvenIndex<'id>> {
        if ix < self.0.len() {
            Some(ProvenIndex(ix, InvariantLifetime::default()))
        } else {
            None
        }
    }

    fn get_proven(&self, ix: &ProvenIndex<'id>) -> u8 {
        self.0[ix.0]
    }
}

fn main() {
    let result = Bytes::new(vec![4, 5, 1], move |mut bytes_1| {
        Bytes::new(vec![4, 2], move |mut bytes_2| {
            let index_1 = bytes_1.get_index(2).unwrap();
            let index_2 = bytes_2.get_index(1).unwrap();
            bytes_1.get_proven(&index_1);
            bytes_2.get_proven(&index_2);
            // bytes_2.get_proven(&index_1); // ❌🔨
            "Computations done!"
        })
    });
    println!("{result}");
}
```

<details>

- We now have the implementation ready, we can now write a program where token
  types that are proofs of existing indexes cannot be shared between variables.

- Demonstration: Uncomment the `bytes_2.get_proven(&index_1);` line and show
  that it does not compile when we use indexes from different variables.

- Ask: What operations can we perform that we can guarantee would produce a
  proven index?

  Expect a "push" implementation, suggested demo:

  ```rust,compile_fail
  fn push(&mut self, value: u8) -> ProvenIndex<'id> {
      self.0.push(value);
      ProvenIndex(self.0.len() - 1, InvariantLifetime::default())
  }
  ```

- Ask: Can we make this not just about a byte array, but as a general wrapper on
  `Vec<T>`?

  Trivial: Yes!

  Maybe demonstrate: Generalising `Bytes<'id>` into `BrandedVec<'id, T>`

- Ask: What other areas could we use something like this?

- The resulting token API is **highly restrictive**, but the things that it
  makes possible to prove as safe within the Rust type system are meaningful.

## More to Explore

- [GhostCell](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf), a structure
  that allows for safe cyclic data structures in Rust (among other previously
  difficult to represent data structures), uses this kind of token type to make
  sure cells can't "escape" a context where we know where operations similar to
  those shown in these examples are safe.

  This "Branded Types" sequence of slides is based off their `BrandedVec`
  implementation in the paper, which covers many of the implementation details
  of this use case in more depth as a gentle introduction to how `GhostCell`
  itself is implemented and used in practice.

  GhostCell also uses formal checks outside of Rust's type system to prove that
  the things it allows within this kind of context (lifetime branding) are safe.

</details>

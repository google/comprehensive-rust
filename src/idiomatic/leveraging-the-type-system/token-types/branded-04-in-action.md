---
minutes: 15
---

# Branded Types in Action (Branding 4/4)

```rust,editable,compile_fail
use std::marker::PhantomData;

#[derive(Default)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

struct BrandedToken<'id>(InvariantLifetime<'id>);
struct MyStructure<'id>(Vec<u8>, InvariantLifetime<'id>);

impl<'id> MyStructure<'id> {
    fn new<T>(
        data: Vec<u8>,
        f: impl for<'a> FnOnce(MyStructure<'a>, BrandedToken<'a>) -> T,
    ) -> T {
        f(
            MyStructure(data, InvariantLifetime::default()),
            BrandedToken(InvariantLifetime::default()),
        )
    }
    fn use_token(&mut self, token: &BrandedToken<'id>) {}
}

fn main() {
    // Work has to happen in the closures! not outside them.
    let result = MyStructure::new(vec![4, 5, 1], move |mut data_1, token_1| {
        MyStructure::new(vec![4, 2], move |mut data_2, token_2| {
            data_1.use_token(&token_1);
            data_2.use_token(&token_2);
            // data_2.use_token(&token_1); // ❌🔨
            "Computations done!"
        })
    });
    println!("{result}");
}
```

<details>

- To use values of these branded types, we will need to use closures instead of
  usual `let` declarations of variables.

  This is clunky, and does lead to indentation drift if you need to use many
  different branded types. But this case is not common.

- Note: The data structures we end up passing to the closures cannot be returned
  on their own.

  The intent being you do the computation you need to do within these closures
  then return a result.

- Note: Show how uncommenting `data_2.use_token(&token_1)` makes this not
  compile.

  Talk about how this is because the lifetimes cannot be subtyped on one
  another, because of how we used `for<'a>`.

- The resulting token API is **highly restrictive**, but the things that it
  makes possible to prove as safe within the rust type system are meaningful.

  [GhostCell](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf), a structure
  that allows for safe cyclic data structures in rust, uses this kind of token
  type to make sure cells can't "escape" a context where we know where cyclic
  operations are safe.

  The token acts like a "proof of arena allocation and destruction" in this
  case. The data structure cannot live past the closure in any way.

  GhostCell uses formal checks outside of Rust's type system to prove that the
  things it allows within this kind of context (cyclic references) are safe.

</details>

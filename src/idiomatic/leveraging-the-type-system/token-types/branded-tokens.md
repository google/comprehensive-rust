---
minutes: 0
---

# Lifetime-branded tokens

We can tie a token to a specific value by using lifetimes.

```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
pub struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

pub struct BrandedToken<'id>(InvariantLifetime<'id>);
pub struct MyStructure<'id>(Vec<u8>, InvariantLifetime<'id>);

impl <'id> MyStructure<'id> {
    pub fn new<T>(
        data: Vec<u8>, 
        f: impl for<'a> FnOnce(MyStructure<'a>, BrandedToken<'a>) -> T
    ) -> T { 
        f(
            MyStructure(
                data, 
                InvariantLifetime::default(),
            ),
            BrandedToken(InvariantLifetime::default()),
        ) 
    }
    pub fn use_token(&mut self, token: &BrandedToken<'id>) {
        println!("This computation needs a token.");
    }
}

fn main() {
    // Work has to happen in the closures! not outside them.
    let result = MyStructure::new(vec![4, 5, 1], move |mut data, token| {
        MyStructure::new(vec![4, 2], move |mut data2, token2| {
            data.use_token(&token);
            data2.use_token(&token2);
            data2.use_token(&token); // ❌🔨
            "Computations done!"
        })
    });
    println!("{result}");
}
```

<details>

- Here we build "branded tokens" that can only be used with the value that constructed them. Branding here in the sense of "being marked with something it is closely associated with."

- Other token types tend to be trivial, this kind of token is at the extreme end of the rust type system, not a day to day occurrence in most rust codebases.

- The "constructor" for `MyStructure` asks for 1. data for the inner value 2. a closure that can only be run once as one of its arguments. 

  Instead of the "constructor" returning a `MyStructure` value, it passes that value and a token tied to that specific value to the closure.

  We end up doing whatever work we wanted to do with this structure _in the closure_ then return some result.

- The token here proves we're "in the closure" as the lifetime parameters mean these values cannot "escape" the closure without being transformed or consumed into some other type.

- The `for<'a>` in the `impl for<'a> FnOnce(...) -> T` type and the type parameter of `InvariantLifetime`'s internal `PhantomData` relates to [Subtyping](https://doc.rust-lang.org/stable/reference/subtyping.html) is what forces this "branding" between lifetimes to apply. Without it, the compiler would see the lifetimes on the types we're handling as "similar enough" (able to be subtyped) and users would be able to use the token for one structure with a different structure.

- This kind of token is **highly restrictive**, but the things that it makes possible to prove as safe within the rust type system are meaningful. The data structures we end up passing to the closures cannot be returned on their own.

  [GhostCell](https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf), a structure that allows for safe cyclic data structures in rust, uses this principle to make sure cells can't "escape" a context where we know where cyclic operations are safe.

</details>
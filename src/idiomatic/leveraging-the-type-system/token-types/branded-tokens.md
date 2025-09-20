---
minutes: 0
---

# Lifetime-branded tokens

We can tie a token to a specific value by using lifetimes.

```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
pub struct InvariantLifetime<'id>(PhantomData<fn(&'id ()) -> &'id ()>);

pub struct BrandedToken<'id>(InvariantLifetime<'id>);

pub struct MyStructure<'id>(Vec<u8>, InvariantLifetime<'id>);

impl <'id> MyStructure<'id> {
    fn new<T>(
        data: impl IntoIterator<Item=u8>, 
        f: impl for<'a> FnOnce(MyStructure<'a>) -> T  ) 
    -> T { 
        f(
            MyStructure(
                data.into_iter().collect(), 
                InvariantLifetime::default(),
            )
        ) 
        }
    fn get_token(&self) -> BrandedToken<'id> {
        BrandedToken(InvariantLifetime::default())
    }
    fn use_token(&self, token: &BrandedToken<'id>) {}
    fn push(&mut self, value: u8) {
        self.0.push(value)
    }
}

fn main() {
    let one = MyStructure::new([4, 5, 1], move |data| {
        MyStructure::new([4, 2], move |data2| {
            let token_1 = data.get_token();
            data.use_token(&token_1);
            let token_2 = data2.get_token();
            data2.use_token(&token_2);
            data2.use_token(&token_1); // ❌🔨
        })
    });   
}
```

<details>

- In this example we build "branded tokens" that can only be used with the value that constructed them.

- This kind of token is HIGHLY restrictive, but the things that it makes possible to prove as safe within the rust type system are meaningful.

- `InvariantLifetime` is a newtype over a `PhantomType`, letting us capture a lifetime in the later

- We can force tokens to only apply to a specific value by "branding" them with lifetimes.

</details>
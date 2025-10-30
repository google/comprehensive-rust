---
minutes: 5
---

# Lifetimes in Function Calls

Lifetimes for function arguments and return values must be fully specified, but
Rust allows lifetimes to be elided in most cases with
[a few simple rules](https://doc.rust-lang.org/nomicon/lifetime-elision.html).
This is not inference -- it is just a syntactic shorthand.

- Each argument which does not have a lifetime annotation is given one.
- If there is only one argument lifetime, it is given to all un-annotated return
  values.
- If there are multiple argument lifetimes, but the first one is for `self`,
  that lifetime is given to all un-annotated return values.

```rust,editable
fn only_args(a: &i32, b: &i32) { todo!(); }

fn identity(a: &i32) -> &i32 { a }

struct Foo(i32);
impl Foo {
    fn get(&self, other: &i32) -> &i32 {
        &self.0
    }
}
```

<details>

- Walk through applying the lifetime elision rules to each of the example
  functions. `only_args` is completed by the first rule, `identity` is completed
  by the second, and `Foo::get` is completed by the third.

- If all lifetimes have not been filled in by applying the three elision rules
  then you will get a compiler error telling you to add annotations manually.

</details>

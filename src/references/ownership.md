---
minutes: 5
---

# Ownership

We already mentioned that one of the main selling points of rust is compile time
memory safety, _without_ garbage collection. In order to achieve this, rust
imposes some very important rules on how values can be used. These rules are
collectively known as the concept of _Ownership_.

We will briefly discuss this here, so that you know about. But don't worry if
you don't immediately feel comfortable with it! It is going to come up again
multiple times throughout the course.

With the above disclaimer, let's see an example where the concept of Ownership
comes into play:

```rust,editable,compile_fail
fn main() {
    let s = String::new();
    let _y = s;
    println!("{s}");
}
```

If you run the code above, you will see that it fails to compile. Most likely
the error messages will seem quite cryptic to you at this point. That is O.K.!
Just focus on the fact that rust won't let you do this.

And this is because of Ownership: Initially `s` is the owner of the `String` we
created. With `let _y = s;`, we moved the ownership of the `String` from `s` to
`_y`. Then, when we try to use `s`, rust won't let us, since it doesn't own a
value to be used any more.

There is still a bunch of things missing to have a complete understanding of
Ownership. But we have seen enough to understand why the next part, References,
is another important concept in rust.

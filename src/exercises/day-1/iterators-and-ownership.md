# Iterators and Ownership

The ownership model of Rust affects many APIs. An example of this is the
[`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and
[`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
traits.

## `Iterator`

Traits are like interfaces: they describe behavior (methods) for a type. The
`Iterator` trait simply says that you can call `next` until you get `None` back:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

You use this trait like this:

```rust,editable
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}
```

What is the type returned by the iterator? Test your answer here:

```rust,editable,compile_fail
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    let v0: Option<..> = iter.next();
    println!("v0: {v0:?}");
}
```

Why is this type used?

## `IntoIterator`

The `Iterator` trait tells you how to _iterate_ once you have created an
iterator. The related trait `IntoIterator` tells you how to create the iterator:

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

The syntax here means that every implementation of `IntoIterator` must
declare two types:

* `Item`: the type we iterate over, such as `i8`,
* `IntoIter`: the `Iterator` type returned by the `into_iter` method.

Note that `IntoIter` and `Item` are linked: the iterator must have the same
`Item` type, which means that it returns `Option<Item>`

Like before, what  is the type returned by the iterator?

```rust,editable,compile_fail
fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<..> = iter.next();
    println!("v0: {v0:?}");
}
```

## `for` Loops

Now that we know both `Iterator` and `IntoIterator`, we can build `for` loops.
They call `into_iter()` on an expression and iterates over the resulting
iterator:

```rust,editable
fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
}
```

What is the type of `word` in each loop?

Experiment with the code above and then consult the documentation for [`impl
IntoIterator for
&Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-%26'a+Vec%3CT,+A%3E)
and [`impl IntoIterator for
Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-Vec%3CT,+A%3E)
to check your answers.

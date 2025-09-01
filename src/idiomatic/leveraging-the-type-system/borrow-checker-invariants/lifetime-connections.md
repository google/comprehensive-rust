---
minutes: 0
---

# Lifetime "Connections" & External Resources

Using `PhantomData` in conjunction with lifetimes lets us say "this value may own its data, but it can only live as long as the value that generated it" in rust's type system.

```rust,editable
fn main() {
use std::marker::PhantomData;
pub struct Tag;
pub struct ErasedData<'a>{data: String, _phantom: PhantomData<&'a ()>};
impl <'a> ErasedData<'a>  {
    pub fn get(&self) -> &str {
        &self.data
    }
}
pub struct TaggedData<T>{data: String, _phantom: PhantomData<T>};
impl <T> TaggedData<T> {
    pub fn new(data: String) -> Self {Self {data, _phantom: PhantomData} }
    pub fn consume(self) {}
    pub fn get_erased(&self) -> ErasedData<'_> {
        // has an owned String, but _phantom holds onto the lifetime of the TaggedData
        // that created it.
        ErasedData { data: self.data.clone(), _phantom: PhantomData }
    }
}

let tagged_data: TaggedData<Tag> = TaggedData::new("Real Data".to_owned());
// Get the erased-but-still-linked data.
let erased_owned_and_linked = tagged_data.get_erased();
tagged_data.consume();
// The data is owned by `erased_owned_and_linked` but still connected to `tagged_data`.
println!("{}", erased_owned_and_linked.get()); // ❌🔨
}
```

<details>

- PhantomData lets developers "tag" types with type and lifetime parameters that are not "really" present in the struct or enum.

- PhantomData can be used with the Typestate pattern to have data with the same structure i.e. `TaggedData<Start>` can have methods or trait implementations that `TaggedData<End>` doesn't.

- But it can also be used to encode a connection between the lifetime of one value and another, while both values still maintain separate owned data within them.

- This is really useful for modelling a bunch of relationships between data, where we want to establish that while a type has owned values within it is still connected to another piece of data and can only live as long as it.

- Consider a case where you want to return owned data from a method, but you don't want that data to live longer than the value that created it.

- `BorrowedFd` / `OwnedFd` uses these captured lifetimes to enforce the invariant that "if this file descriptor exists, the OS file descriptor is still open" because a `BorrowedFd`'s lifetime parameter demands that there exists another value in your program that has the same lifetime as it, and this has been encoded by the API designer to mean _that value is what keeps the access to the file open_.

- Lifetimes need to come from somewhere! We can't build functions of the form `fn lifetime_shenanigans<'a>(owned: OwnedData) -> &'b Data`. Most often a lifetime comes from the lifetime `'a` from `&'a self` in a method, but not always!

- This encoding is _exceptionally powerful_ when combined with unsafe, as the ways one can manipulate lifetimes becomes almost arbitrary. This is dangerous, but when combined with tools like formal, mechanically-verified proofs we can _safely encode cyclic types_ (see: the GhostCell paper.)

</details>